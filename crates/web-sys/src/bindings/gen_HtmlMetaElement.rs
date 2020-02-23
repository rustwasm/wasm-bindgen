use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLMetaElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlMetaElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlMetaElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlMetaElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(77u32);
            inform(101u32);
            inform(116u32);
            inform(97u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlMetaElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlMetaElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlMetaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlMetaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlMetaElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlMetaElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlMetaElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlMetaElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlMetaElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlMetaElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlMetaElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlMetaElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlMetaElement {
            HtmlMetaElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlMetaElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlMetaElement> for HtmlMetaElement {
        #[inline]
        fn as_ref(&self) -> &HtmlMetaElement {
            self
        }
    }
    impl From<HtmlMetaElement> for JsValue {
        #[inline]
        fn from(obj: HtmlMetaElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlMetaElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLMetaElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLMetaElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLMetaElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlMetaElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlMetaElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlMetaElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlMetaElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlMetaElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMetaElement> for Element {
    #[inline]
    fn from(obj: HtmlMetaElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlMetaElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMetaElement> for Node {
    #[inline]
    fn from(obj: HtmlMetaElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlMetaElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMetaElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlMetaElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlMetaElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMetaElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlMetaElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlMetaElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLMetaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/name)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLMetaElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_http_equiv_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `httpEquiv` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn http_equiv(&self) -> String {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_http_equiv_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_http_equiv_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_http_equiv_HTMLMetaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_http_equiv_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `httpEquiv` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/httpEquiv)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn set_http_equiv(&self, http_equiv: &str) {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_http_equiv_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                http_equiv: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_http_equiv_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            http_equiv: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(http_equiv);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let http_equiv = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(http_equiv);
                __widl_f_set_http_equiv_HTMLMetaElement(self_, http_equiv)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `content` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn content(&self) -> String {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_HTMLMetaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_content_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `content` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/content)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn set_content(&self, content: &str) {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_content_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                content: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_content_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            content: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(content);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let content = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(content);
                __widl_f_set_content_HTMLMetaElement(self_, content)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_scheme_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `scheme` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn scheme(&self) -> String {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_scheme_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_scheme_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_scheme_HTMLMetaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMetaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_scheme_HTMLMetaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMetaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMetaElement {
    #[cfg(all(feature = "HtmlMetaElement",))]
    #[allow(bad_style)]
    #[doc = "The `scheme` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMetaElement/scheme)\n\n*This API requires the following crate features to be activated: `HtmlMetaElement`*"]
    #[allow(clippy::all)]
    pub fn set_scheme(&self, scheme: &str) {
        #[cfg(all(feature = "HtmlMetaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_scheme_HTMLMetaElement(
                self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                scheme: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_scheme_HTMLMetaElement(
            self_: <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            scheme: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(scheme);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMetaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let scheme = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(scheme);
                __widl_f_set_scheme_HTMLMetaElement(self_, scheme)
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
pub static __WASM_BINDGEN_GENERATED_9b41d93658b88f6d: [u8; 860usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1A\x03\0\0\0\0\t\0\0\x02\x0FHTMLMetaElement!__widl_instanceof_HTMLMetaElement\0\0\0\0\x1D__widl_f_name_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0!__widl_f_set_name_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0#__widl_f_http_equiv_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x01\thttpEquiv\x01\x01\x05self_\thttpEquiv\0\0\0'__widl_f_set_http_equiv_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x02\thttpEquiv\x01\x02\x05self_\nhttp_equiv\thttpEquiv\0\0\0 __widl_f_content_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x01\x07content\x01\x01\x05self_\x07content\0\0\0$__widl_f_set_content_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x02\x07content\x01\x02\x05self_\x07content\x07content\0\0\0\x1F__widl_f_scheme_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x01\x06scheme\x01\x01\x05self_\x06scheme\0\0\0#__widl_f_set_scheme_HTMLMetaElement\0\0\0\x01\x0FHTMLMetaElement\x01\0\x02\x06scheme\x01\x02\x05self_\x06scheme\x06scheme\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
