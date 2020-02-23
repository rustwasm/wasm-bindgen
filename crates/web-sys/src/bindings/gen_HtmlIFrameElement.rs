use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLIFrameElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlIFrameElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlIFrameElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlIFrameElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(73u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
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
    impl core::ops::Deref for HtmlIFrameElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlIFrameElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlIFrameElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlIFrameElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlIFrameElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlIFrameElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlIFrameElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlIFrameElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlIFrameElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlIFrameElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlIFrameElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlIFrameElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlIFrameElement {
            HtmlIFrameElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlIFrameElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlIFrameElement> for HtmlIFrameElement {
        #[inline]
        fn as_ref(&self) -> &HtmlIFrameElement {
            self
        }
    }
    impl From<HtmlIFrameElement> for JsValue {
        #[inline]
        fn from(obj: HtmlIFrameElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlIFrameElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLIFrameElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLIFrameElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLIFrameElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlIFrameElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlIFrameElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlIFrameElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlIFrameElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlIFrameElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlIFrameElement> for Element {
    #[inline]
    fn from(obj: HtmlIFrameElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlIFrameElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlIFrameElement> for Node {
    #[inline]
    fn from(obj: HtmlIFrameElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlIFrameElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlIFrameElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlIFrameElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlIFrameElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlIFrameElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlIFrameElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlIFrameElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Document", feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_svg_document_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "Document", feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `getSVGDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/getSVGDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn get_svg_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_svg_document_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_svg_document_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_svg_document_HTMLIFrameElement(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/src)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLIFrameElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_srcdoc_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `srcdoc` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn srcdoc(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_srcdoc_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_srcdoc_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_srcdoc_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_srcdoc_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `srcdoc` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/srcdoc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_srcdoc(&self, srcdoc: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_srcdoc_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                srcdoc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_srcdoc_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            srcdoc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(srcdoc);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let srcdoc = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(srcdoc);
                __widl_f_set_srcdoc_HTMLIFrameElement(self_, srcdoc)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/name)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLIFrameElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sandbox_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "DomTokenList", feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `sandbox` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/sandbox)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn sandbox(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sandbox_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sandbox_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sandbox_HTMLIFrameElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_allow_fullscreen_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `allowFullscreen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn allow_fullscreen(&self) -> bool {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_allow_fullscreen_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_allow_fullscreen_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_allow_fullscreen_HTMLIFrameElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_allow_fullscreen_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `allowFullscreen` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowFullscreen)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_allow_fullscreen(&self, allow_fullscreen: bool) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_allow_fullscreen_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                allow_fullscreen: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_allow_fullscreen_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            allow_fullscreen: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(allow_fullscreen);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let allow_fullscreen =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(allow_fullscreen);
                __widl_f_set_allow_fullscreen_HTMLIFrameElement(self_, allow_fullscreen)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_allow_payment_request_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `allowPaymentRequest` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn allow_payment_request(&self) -> bool {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_allow_payment_request_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_allow_payment_request_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_allow_payment_request_HTMLIFrameElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_allow_payment_request_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `allowPaymentRequest` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/allowPaymentRequest)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_allow_payment_request(&self, allow_payment_request: bool) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_allow_payment_request_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                allow_payment_request: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_allow_payment_request_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            allow_payment_request: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(allow_payment_request);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let allow_payment_request =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(allow_payment_request);
                __widl_f_set_allow_payment_request_HTMLIFrameElement(self_, allow_payment_request)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/width)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLIFrameElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/height)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLIFrameElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_referrer_policy_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_referrer_policy(&self, referrer_policy: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_referrer_policy_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_referrer_policy_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(referrer_policy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let referrer_policy =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(referrer_policy);
                __widl_f_set_referrer_policy_HTMLIFrameElement(self_, referrer_policy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_document_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "Document", feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `contentDocument` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn content_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_document_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_document_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_document_HTMLIFrameElement(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_window_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `contentWindow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/contentWindow)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`, `Window`*"]
    #[allow(clippy::all)]
    pub fn content_window(&self) -> Option<Window> {
        #[cfg(all(feature = "HtmlIFrameElement", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_window_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_window_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_window_HTMLIFrameElement(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/align)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLIFrameElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scrolling_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `scrolling` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn scrolling(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scrolling_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scrolling_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scrolling_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scrolling_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `scrolling` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/scrolling)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_scrolling(&self, scrolling: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scrolling_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scrolling: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scrolling_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scrolling: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scrolling);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scrolling = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scrolling);
                __widl_f_set_scrolling_HTMLIFrameElement(self_, scrolling)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frame_border_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `frameBorder` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn frame_border(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frame_border_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frame_border_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frame_border_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_frame_border_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `frameBorder` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/frameBorder)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_frame_border(&self, frame_border: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_frame_border_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                frame_border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_frame_border_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            frame_border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(frame_border);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let frame_border =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(frame_border);
                __widl_f_set_frame_border_HTMLIFrameElement(self_, frame_border)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_long_desc_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `longDesc` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn long_desc(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_long_desc_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_long_desc_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_long_desc_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_long_desc_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `longDesc` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/longDesc)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_long_desc(&self, long_desc: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_long_desc_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                long_desc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_long_desc_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            long_desc: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(long_desc);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let long_desc = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(long_desc);
                __widl_f_set_long_desc_HTMLIFrameElement(self_, long_desc)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_margin_height_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn margin_height(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_margin_height_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_margin_height_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_margin_height_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_margin_height_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginHeight` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginHeight)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_margin_height(&self, margin_height: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_margin_height_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                margin_height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_margin_height_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            margin_height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(margin_height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let margin_height =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(margin_height);
                __widl_f_set_margin_height_HTMLIFrameElement(self_, margin_height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_margin_width_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn margin_width(&self) -> String {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_margin_width_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_margin_width_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_margin_width_HTMLIFrameElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlIFrameElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_margin_width_HTMLIFrameElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlIFrameElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlIFrameElement {
    #[cfg(all(feature = "HtmlIFrameElement",))]
    #[allow(bad_style)]
    #[doc = "The `marginWidth` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLIFrameElement/marginWidth)\n\n*This API requires the following crate features to be activated: `HtmlIFrameElement`*"]
    #[allow(clippy::all)]
    pub fn set_margin_width(&self, margin_width: &str) {
        #[cfg(all(feature = "HtmlIFrameElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_margin_width_HTMLIFrameElement(
                self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                margin_width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_margin_width_HTMLIFrameElement(
            self_: <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            margin_width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(margin_width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlIFrameElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let margin_width =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(margin_width);
                __widl_f_set_margin_width_HTMLIFrameElement(self_, margin_width)
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
pub static __WASM_BINDGEN_GENERATED_1c4f94dfff3e3e58: [u8; 3401usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x07\r\0\0\0\0!\0\0\x02\x11HTMLIFrameElement#__widl_instanceof_HTMLIFrameElement\0\0\0\0+__widl_f_get_svg_document_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\0\x01\x01\x05self_\x0EgetSVGDocument\0\0\0\x1E__widl_f_src_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0\"__widl_f_set_src_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0!__widl_f_srcdoc_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x06srcdoc\x01\x01\x05self_\x06srcdoc\0\0\0%__widl_f_set_srcdoc_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x06srcdoc\x01\x02\x05self_\x06srcdoc\x06srcdoc\0\0\0\x1F__widl_f_name_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_set_name_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\"__widl_f_sandbox_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x07sandbox\x01\x01\x05self_\x07sandbox\0\0\0+__widl_f_allow_fullscreen_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x0FallowFullscreen\x01\x01\x05self_\x0FallowFullscreen\0\0\0/__widl_f_set_allow_fullscreen_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x0FallowFullscreen\x01\x02\x05self_\x10allow_fullscreen\x0FallowFullscreen\0\0\00__widl_f_allow_payment_request_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x13allowPaymentRequest\x01\x01\x05self_\x13allowPaymentRequest\0\0\04__widl_f_set_allow_payment_request_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x13allowPaymentRequest\x01\x02\x05self_\x15allow_payment_request\x13allowPaymentRequest\0\0\0 __widl_f_width_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0$__widl_f_set_width_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0!__widl_f_height_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0%__widl_f_set_height_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0*__widl_f_referrer_policy_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0.__widl_f_set_referrer_policy_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x0EreferrerPolicy\x01\x02\x05self_\x0Freferrer_policy\x0EreferrerPolicy\0\0\0+__widl_f_content_document_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x0FcontentDocument\x01\x01\x05self_\x0FcontentDocument\0\0\0)__widl_f_content_window_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\rcontentWindow\x01\x01\x05self_\rcontentWindow\0\0\0 __widl_f_align_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0$__widl_f_set_align_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0$__widl_f_scrolling_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\tscrolling\x01\x01\x05self_\tscrolling\0\0\0(__widl_f_set_scrolling_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\tscrolling\x01\x02\x05self_\tscrolling\tscrolling\0\0\0'__widl_f_frame_border_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x0BframeBorder\x01\x01\x05self_\x0BframeBorder\0\0\0+__widl_f_set_frame_border_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x0BframeBorder\x01\x02\x05self_\x0Cframe_border\x0BframeBorder\0\0\0$__widl_f_long_desc_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x08longDesc\x01\x01\x05self_\x08longDesc\0\0\0(__widl_f_set_long_desc_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x08longDesc\x01\x02\x05self_\tlong_desc\x08longDesc\0\0\0(__widl_f_margin_height_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x0CmarginHeight\x01\x01\x05self_\x0CmarginHeight\0\0\0,__widl_f_set_margin_height_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x0CmarginHeight\x01\x02\x05self_\rmargin_height\x0CmarginHeight\0\0\0'__widl_f_margin_width_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x01\x0BmarginWidth\x01\x01\x05self_\x0BmarginWidth\0\0\0+__widl_f_set_margin_width_HTMLIFrameElement\0\0\0\x01\x11HTMLIFrameElement\x01\0\x02\x0BmarginWidth\x01\x02\x05self_\x0Cmargin_width\x0BmarginWidth\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
