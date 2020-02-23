use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLBodyElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlBodyElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlBodyElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlBodyElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(66u32);
            inform(111u32);
            inform(100u32);
            inform(121u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlBodyElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlBodyElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlBodyElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlBodyElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlBodyElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlBodyElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlBodyElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlBodyElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlBodyElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlBodyElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlBodyElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlBodyElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlBodyElement {
            HtmlBodyElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlBodyElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlBodyElement> for HtmlBodyElement {
        #[inline]
        fn as_ref(&self) -> &HtmlBodyElement {
            self
        }
    }
    impl From<HtmlBodyElement> for JsValue {
        #[inline]
        fn from(obj: HtmlBodyElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlBodyElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLBodyElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLBodyElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLBodyElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlBodyElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlBodyElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlBodyElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlBodyElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlBodyElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBodyElement> for Element {
    #[inline]
    fn from(obj: HtmlBodyElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlBodyElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBodyElement> for Node {
    #[inline]
    fn from(obj: HtmlBodyElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlBodyElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBodyElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlBodyElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlBodyElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBodyElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlBodyElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlBodyElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> String {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_HTMLBodyElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/text)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_HTMLBodyElement(self_, text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_link_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `link` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn link(&self) -> String {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_link_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_link_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_link_HTMLBodyElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_link_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `link` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/link)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_link(&self, link: &str) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_link_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                link: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_link_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            link: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(link);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let link = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(link);
                __widl_f_set_link_HTMLBodyElement(self_, link)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_v_link_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `vLink` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn v_link(&self) -> String {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_v_link_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_v_link_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_v_link_HTMLBodyElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_v_link_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `vLink` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/vLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_v_link(&self, v_link: &str) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_v_link_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                v_link: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_v_link_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            v_link: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(v_link);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let v_link = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(v_link);
                __widl_f_set_v_link_HTMLBodyElement(self_, v_link)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_a_link_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `aLink` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn a_link(&self) -> String {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_a_link_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_a_link_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_a_link_HTMLBodyElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_a_link_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `aLink` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/aLink)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_a_link(&self, a_link: &str) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_a_link_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                a_link: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_a_link_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            a_link: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(a_link);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let a_link = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(a_link);
                __widl_f_set_a_link_HTMLBodyElement(self_, a_link)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_bg_color_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn bg_color(&self) -> String {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_bg_color_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_bg_color_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_bg_color_HTMLBodyElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_bg_color_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `bgColor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/bgColor)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_bg_color(&self, bg_color: &str) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_bg_color_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_bg_color_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            bg_color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(bg_color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let bg_color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(bg_color);
                __widl_f_set_bg_color_HTMLBodyElement(self_, bg_color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_background_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `background` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn background(&self) -> String {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_background_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_background_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_background_HTMLBodyElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_background_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `background` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/background)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_background(&self, background: &str) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_background_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                background: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_background_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            background: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(background);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let background = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(background);
                __widl_f_set_background_HTMLBodyElement(self_, background)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onafterprint_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onafterprint` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onafterprint(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onafterprint_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onafterprint_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onafterprint_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onafterprint_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onafterprint` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onafterprint(&self, onafterprint: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onafterprint_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onafterprint : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onafterprint_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onafterprint: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onafterprint);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onafterprint =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onafterprint,
                    );
                __widl_f_set_onafterprint_HTMLBodyElement(self_, onafterprint)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onbeforeprint_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeprint` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onbeforeprint(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onbeforeprint_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onbeforeprint_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onbeforeprint_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onbeforeprint_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeprint` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onbeforeprint(&self, onbeforeprint: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onbeforeprint_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onbeforeprint : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onbeforeprint_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onbeforeprint: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onbeforeprint);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onbeforeprint =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onbeforeprint,
                    );
                __widl_f_set_onbeforeprint_HTMLBodyElement(self_, onbeforeprint)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onbeforeunload_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeunload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onbeforeunload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onbeforeunload_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onbeforeunload_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onbeforeunload_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onbeforeunload_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeunload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onbeforeunload(&self, onbeforeunload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onbeforeunload_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onbeforeunload : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onbeforeunload_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onbeforeunload : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onbeforeunload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onbeforeunload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onbeforeunload,
                    );
                __widl_f_set_onbeforeunload_HTMLBodyElement(self_, onbeforeunload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onhashchange_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onhashchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onhashchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onhashchange_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onhashchange_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onhashchange_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onhashchange_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onhashchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onhashchange(&self, onhashchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onhashchange_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onhashchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onhashchange_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onhashchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onhashchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onhashchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onhashchange,
                    );
                __widl_f_set_onhashchange_HTMLBodyElement(self_, onhashchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onlanguagechange_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlanguagechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onlanguagechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onlanguagechange_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onlanguagechange_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onlanguagechange_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onlanguagechange_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlanguagechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onlanguagechange(&self, onlanguagechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onlanguagechange_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onlanguagechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onlanguagechange_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onlanguagechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onlanguagechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onlanguagechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onlanguagechange,
                    );
                __widl_f_set_onlanguagechange_HTMLBodyElement(self_, onlanguagechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_HTMLBodyElement(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessageerror_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onmessageerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessageerror_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessageerror_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessageerror_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessageerror_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmessageerror(&self, onmessageerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessageerror_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessageerror_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onmessageerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessageerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessageerror,
                    );
                __widl_f_set_onmessageerror_HTMLBodyElement(self_, onmessageerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onoffline_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onoffline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onoffline(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onoffline_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onoffline_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onoffline_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onoffline_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onoffline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onoffline(&self, onoffline: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onoffline_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onoffline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onoffline_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onoffline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onoffline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onoffline =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onoffline,
                    );
                __widl_f_set_onoffline_HTMLBodyElement(self_, onoffline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ononline_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `ononline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn ononline(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ononline_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ononline_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ononline_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ononline_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `ononline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_ononline(&self, ononline: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ononline_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ononline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ononline_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ononline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ononline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ononline =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ononline,
                    );
                __widl_f_set_ononline_HTMLBodyElement(self_, ononline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpagehide_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpagehide` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onpagehide(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpagehide_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpagehide_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpagehide_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpagehide_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpagehide` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpagehide(&self, onpagehide: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpagehide_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpagehide : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpagehide_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpagehide: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpagehide);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpagehide =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpagehide,
                    );
                __widl_f_set_onpagehide_HTMLBodyElement(self_, onpagehide)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpageshow_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpageshow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onpageshow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpageshow_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpageshow_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpageshow_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpageshow_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpageshow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpageshow(&self, onpageshow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpageshow_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpageshow : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpageshow_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpageshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpageshow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpageshow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpageshow,
                    );
                __widl_f_set_onpageshow_HTMLBodyElement(self_, onpageshow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpopstate_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpopstate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onpopstate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpopstate_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpopstate_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpopstate_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpopstate_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpopstate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpopstate(&self, onpopstate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpopstate_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpopstate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpopstate_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpopstate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpopstate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpopstate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpopstate,
                    );
                __widl_f_set_onpopstate_HTMLBodyElement(self_, onpopstate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstorage_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstorage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onstorage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstorage_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstorage_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstorage_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstorage_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstorage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onstorage(&self, onstorage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstorage_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstorage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstorage_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstorage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstorage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstorage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstorage,
                    );
                __widl_f_set_onstorage_HTMLBodyElement(self_, onstorage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onunload_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onunload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn onunload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onunload_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onunload_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onunload_HTMLBodyElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBodyElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onunload_HTMLBodyElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBodyElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBodyElement {
    #[cfg(all(feature = "HtmlBodyElement",))]
    #[allow(bad_style)]
    #[doc = "The `onunload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlBodyElement`*"]
    #[allow(clippy::all)]
    pub fn set_onunload(&self, onunload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlBodyElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onunload_HTMLBodyElement(
                self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onunload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onunload_HTMLBodyElement(
            self_: <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onunload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onunload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlBodyElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onunload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onunload,
                    );
                __widl_f_set_onunload_HTMLBodyElement(self_, onunload)
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
pub static __WASM_BINDGEN_GENERATED_e6ec464756a852cf: [u8; 4071usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA5\x0F\0\0\0\0)\0\0\x02\x0FHTMLBodyElement!__widl_instanceof_HTMLBodyElement\0\0\0\0\x1D__widl_f_text_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0!__widl_f_set_text_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0\x1D__widl_f_link_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x04link\x01\x01\x05self_\x04link\0\0\0!__widl_f_set_link_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x04link\x01\x02\x05self_\x04link\x04link\0\0\0\x1F__widl_f_v_link_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x05vLink\x01\x01\x05self_\x05vLink\0\0\0#__widl_f_set_v_link_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x05vLink\x01\x02\x05self_\x06v_link\x05vLink\0\0\0\x1F__widl_f_a_link_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x05aLink\x01\x01\x05self_\x05aLink\0\0\0#__widl_f_set_a_link_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x05aLink\x01\x02\x05self_\x06a_link\x05aLink\0\0\0!__widl_f_bg_color_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x07bgColor\x01\x01\x05self_\x07bgColor\0\0\0%__widl_f_set_bg_color_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x07bgColor\x01\x02\x05self_\x08bg_color\x07bgColor\0\0\0#__widl_f_background_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\nbackground\x01\x01\x05self_\nbackground\0\0\0'__widl_f_set_background_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\nbackground\x01\x02\x05self_\nbackground\nbackground\0\0\0%__widl_f_onafterprint_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x0Conafterprint\x01\x01\x05self_\x0Conafterprint\0\0\0)__widl_f_set_onafterprint_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x0Conafterprint\x01\x02\x05self_\x0Conafterprint\x0Conafterprint\0\0\0&__widl_f_onbeforeprint_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\ronbeforeprint\x01\x01\x05self_\ronbeforeprint\0\0\0*__widl_f_set_onbeforeprint_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\ronbeforeprint\x01\x02\x05self_\ronbeforeprint\ronbeforeprint\0\0\0'__widl_f_onbeforeunload_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x0Eonbeforeunload\x01\x01\x05self_\x0Eonbeforeunload\0\0\0+__widl_f_set_onbeforeunload_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x0Eonbeforeunload\x01\x02\x05self_\x0Eonbeforeunload\x0Eonbeforeunload\0\0\0%__widl_f_onhashchange_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x0Conhashchange\x01\x01\x05self_\x0Conhashchange\0\0\0)__widl_f_set_onhashchange_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x0Conhashchange\x01\x02\x05self_\x0Conhashchange\x0Conhashchange\0\0\0)__widl_f_onlanguagechange_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x10onlanguagechange\x01\x01\x05self_\x10onlanguagechange\0\0\0-__widl_f_set_onlanguagechange_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x10onlanguagechange\x01\x02\x05self_\x10onlanguagechange\x10onlanguagechange\0\0\0\"__widl_f_onmessage_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0&__widl_f_set_onmessage_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0'__widl_f_onmessageerror_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x0Eonmessageerror\x01\x01\x05self_\x0Eonmessageerror\0\0\0+__widl_f_set_onmessageerror_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x0Eonmessageerror\x01\x02\x05self_\x0Eonmessageerror\x0Eonmessageerror\0\0\0\"__widl_f_onoffline_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\tonoffline\x01\x01\x05self_\tonoffline\0\0\0&__widl_f_set_onoffline_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\tonoffline\x01\x02\x05self_\tonoffline\tonoffline\0\0\0!__widl_f_ononline_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x08ononline\x01\x01\x05self_\x08ononline\0\0\0%__widl_f_set_ononline_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x08ononline\x01\x02\x05self_\x08ononline\x08ononline\0\0\0#__widl_f_onpagehide_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\nonpagehide\x01\x01\x05self_\nonpagehide\0\0\0'__widl_f_set_onpagehide_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\nonpagehide\x01\x02\x05self_\nonpagehide\nonpagehide\0\0\0#__widl_f_onpageshow_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\nonpageshow\x01\x01\x05self_\nonpageshow\0\0\0'__widl_f_set_onpageshow_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\nonpageshow\x01\x02\x05self_\nonpageshow\nonpageshow\0\0\0#__widl_f_onpopstate_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\nonpopstate\x01\x01\x05self_\nonpopstate\0\0\0'__widl_f_set_onpopstate_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\nonpopstate\x01\x02\x05self_\nonpopstate\nonpopstate\0\0\0\"__widl_f_onstorage_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\tonstorage\x01\x01\x05self_\tonstorage\0\0\0&__widl_f_set_onstorage_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\tonstorage\x01\x02\x05self_\tonstorage\tonstorage\0\0\0!__widl_f_onunload_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x01\x08onunload\x01\x01\x05self_\x08onunload\0\0\0%__widl_f_set_onunload_HTMLBodyElement\0\0\0\x01\x0FHTMLBodyElement\x01\0\x02\x08onunload\x01\x02\x05self_\x08onunload\x08onunload\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
