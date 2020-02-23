use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLLinkElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlLinkElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlLinkElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlLinkElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(76u32);
            inform(105u32);
            inform(110u32);
            inform(107u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlLinkElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlLinkElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlLinkElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlLinkElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlLinkElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlLinkElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlLinkElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlLinkElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlLinkElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlLinkElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlLinkElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlLinkElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlLinkElement {
            HtmlLinkElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlLinkElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlLinkElement> for HtmlLinkElement {
        #[inline]
        fn as_ref(&self) -> &HtmlLinkElement {
            self
        }
    }
    impl From<HtmlLinkElement> for JsValue {
        #[inline]
        fn from(obj: HtmlLinkElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlLinkElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLLinkElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLLinkElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLLinkElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlLinkElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlLinkElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlLinkElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlLinkElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlLinkElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLinkElement> for Element {
    #[inline]
    fn from(obj: HtmlLinkElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlLinkElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLinkElement> for Node {
    #[inline]
    fn from(obj: HtmlLinkElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlLinkElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLinkElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlLinkElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlLinkElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLinkElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlLinkElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlLinkElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLLinkElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(disabled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLLinkElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_href_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/href)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_href(&self, href: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_href_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_href_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(href);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let href = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(href);
                __widl_f_set_href_HTMLLinkElement(self_, href)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cross_origin_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn cross_origin(&self) -> Option<String> {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cross_origin_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cross_origin_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cross_origin_HTMLLinkElement(self_)
            };
            <Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cross_origin_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `crossOrigin` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/crossOrigin)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_cross_origin(&self, cross_origin: Option<&str>) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cross_origin_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cross_origin_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cross_origin: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cross_origin);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cross_origin =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cross_origin);
                __widl_f_set_cross_origin_HTMLLinkElement(self_, cross_origin)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn rel(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rel_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_rel(&self, rel: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rel_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rel_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rel = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rel);
                __widl_f_set_rel_HTMLLinkElement(self_, rel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_list_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "DomTokenList", feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `relList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn rel_list(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_list_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_list_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_list_HTMLLinkElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_media_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `media` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn media(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_media_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_media_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_media_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_media_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `media` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/media)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_media(&self, media: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_media_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_media_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(media);
                __widl_f_set_media_HTMLLinkElement(self_, media)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hreflang_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `hreflang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn hreflang(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hreflang_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hreflang_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hreflang_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hreflang_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `hreflang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_hreflang(&self, hreflang: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hreflang_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hreflang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hreflang_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hreflang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hreflang);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hreflang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hreflang);
                __widl_f_set_hreflang_HTMLLinkElement(self_, hreflang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/type)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLLinkElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_referrer_policy_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_referrer_policy(&self, referrer_policy: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_referrer_policy_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_referrer_policy_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let referrer_policy =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(referrer_policy);
                __widl_f_set_referrer_policy_HTMLLinkElement(self_, referrer_policy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sizes_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "DomTokenList", feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `sizes` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sizes)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn sizes(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sizes_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sizes_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sizes_HTMLLinkElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_charset_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `charset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn charset(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_charset_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_charset_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_charset_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_charset_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `charset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_charset(&self, charset: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_charset_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_charset_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(charset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charset = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charset);
                __widl_f_set_charset_HTMLLinkElement(self_, charset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rev_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `rev` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn rev(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rev_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rev_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rev_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rev_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `rev` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_rev(&self, rev: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rev_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rev: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rev_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rev: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rev);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rev = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rev);
                __widl_f_set_rev_HTMLLinkElement(self_, rev)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/target)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_target(&self, target: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_set_target_HTMLLinkElement(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_integrity_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `integrity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn integrity(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_integrity_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_integrity_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_integrity_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_integrity_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `integrity` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/integrity)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_integrity(&self, integrity: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_integrity_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                integrity: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_integrity_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            integrity: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(integrity);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let integrity = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(integrity);
                __widl_f_set_integrity_HTMLLinkElement(self_, integrity)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_as_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `as` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn as_(&self) -> String {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_as_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_as_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_as_HTMLLinkElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_as_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement",))]
    #[allow(bad_style)]
    #[doc = "The `as` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/as)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`*"]
    #[allow(clippy::all)]
    pub fn set_as(&self, as_: &str) {
        #[cfg(all(feature = "HtmlLinkElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_as_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                as_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_as_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            as_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(as_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let as_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(as_);
                __widl_f_set_as_HTMLLinkElement(self_, as_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlLinkElement", feature = "StyleSheet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sheet_HTMLLinkElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLinkElement as WasmDescribe>::describe();
    <Option<StyleSheet> as WasmDescribe>::describe();
}
impl HtmlLinkElement {
    #[cfg(all(feature = "HtmlLinkElement", feature = "StyleSheet",))]
    #[allow(bad_style)]
    #[doc = "The `sheet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLinkElement/sheet)\n\n*This API requires the following crate features to be activated: `HtmlLinkElement`, `StyleSheet`*"]
    #[allow(clippy::all)]
    pub fn sheet(&self) -> Option<StyleSheet> {
        #[cfg(all(feature = "HtmlLinkElement", feature = "StyleSheet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sheet_HTMLLinkElement(
                self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<StyleSheet> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sheet_HTMLLinkElement(
            self_: <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLinkElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sheet_HTMLLinkElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_d167730046da72e6: [u8; 2670usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"},\n\0\0\0\0\x1E\0\0\x02\x0FHTMLLinkElement!__widl_instanceof_HTMLLinkElement\0\0\0\0!__widl_f_disabled_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0%__widl_f_set_disabled_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0\x1D__widl_f_href_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0!__widl_f_set_href_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x04href\x01\x02\x05self_\x04href\x04href\0\0\0%__widl_f_cross_origin_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x0BcrossOrigin\x01\x01\x05self_\x0BcrossOrigin\0\0\0)__widl_f_set_cross_origin_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x0BcrossOrigin\x01\x02\x05self_\x0Ccross_origin\x0BcrossOrigin\0\0\0\x1C__widl_f_rel_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x03rel\x01\x01\x05self_\x03rel\0\0\0 __widl_f_set_rel_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x03rel\x01\x02\x05self_\x03rel\x03rel\0\0\0!__widl_f_rel_list_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x07relList\x01\x01\x05self_\x07relList\0\0\0\x1E__widl_f_media_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x05media\x01\x01\x05self_\x05media\0\0\0\"__widl_f_set_media_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x05media\x01\x02\x05self_\x05media\x05media\0\0\0!__widl_f_hreflang_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x08hreflang\x01\x01\x05self_\x08hreflang\0\0\0%__widl_f_set_hreflang_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x08hreflang\x01\x02\x05self_\x08hreflang\x08hreflang\0\0\0\x1D__widl_f_type_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0!__widl_f_set_type_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0(__widl_f_referrer_policy_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0,__widl_f_set_referrer_policy_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x0EreferrerPolicy\x01\x02\x05self_\x0Freferrer_policy\x0EreferrerPolicy\0\0\0\x1E__widl_f_sizes_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x05sizes\x01\x01\x05self_\x05sizes\0\0\0 __widl_f_charset_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x07charset\x01\x01\x05self_\x07charset\0\0\0$__widl_f_set_charset_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x07charset\x01\x02\x05self_\x07charset\x07charset\0\0\0\x1C__widl_f_rev_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x03rev\x01\x01\x05self_\x03rev\0\0\0 __widl_f_set_rev_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x03rev\x01\x02\x05self_\x03rev\x03rev\0\0\0\x1F__widl_f_target_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0#__widl_f_set_target_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x06target\x01\x02\x05self_\x06target\x06target\0\0\0\"__widl_f_integrity_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\tintegrity\x01\x01\x05self_\tintegrity\0\0\0&__widl_f_set_integrity_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\tintegrity\x01\x02\x05self_\tintegrity\tintegrity\0\0\0\x1B__widl_f_as_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x02as\x01\x01\x05self_\x02as\0\0\0\x1F__widl_f_set_as_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x02\x02as\x01\x02\x05self_\x03as_\x02as\0\0\0\x1E__widl_f_sheet_HTMLLinkElement\0\0\0\x01\x0FHTMLLinkElement\x01\0\x01\x05sheet\x01\x01\x05self_\x05sheet\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
