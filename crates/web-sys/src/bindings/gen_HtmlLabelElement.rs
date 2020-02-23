use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLLabelElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlLabelElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlLabelElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlLabelElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(76u32);
            inform(97u32);
            inform(98u32);
            inform(101u32);
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
    impl core::ops::Deref for HtmlLabelElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlLabelElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlLabelElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlLabelElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlLabelElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlLabelElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlLabelElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlLabelElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlLabelElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlLabelElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlLabelElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlLabelElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlLabelElement {
            HtmlLabelElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlLabelElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlLabelElement> for HtmlLabelElement {
        #[inline]
        fn as_ref(&self) -> &HtmlLabelElement {
            self
        }
    }
    impl From<HtmlLabelElement> for JsValue {
        #[inline]
        fn from(obj: HtmlLabelElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlLabelElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLLabelElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLLabelElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLLabelElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlLabelElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlLabelElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlLabelElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlLabelElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlLabelElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLabelElement> for Element {
    #[inline]
    fn from(obj: HtmlLabelElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlLabelElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLabelElement> for Node {
    #[inline]
    fn from(obj: HtmlLabelElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlLabelElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLabelElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlLabelElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlLabelElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlLabelElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlLabelElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlLabelElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlLabelElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLLabelElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLabelElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlLabelElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlLabelElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlLabelElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlLabelElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLLabelElement(
                self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLLabelElement(
            self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLLabelElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLabelElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_html_for_HTMLLabelElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLabelElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlLabelElement {
    #[cfg(all(feature = "HtmlLabelElement",))]
    #[allow(bad_style)]
    #[doc = "The `htmlFor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    #[allow(clippy::all)]
    pub fn html_for(&self) -> String {
        #[cfg(all(feature = "HtmlLabelElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_html_for_HTMLLabelElement(
                self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_html_for_HTMLLabelElement(
            self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_html_for_HTMLLabelElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlLabelElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_html_for_HTMLLabelElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlLabelElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlLabelElement {
    #[cfg(all(feature = "HtmlLabelElement",))]
    #[allow(bad_style)]
    #[doc = "The `htmlFor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)\n\n*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
    #[allow(clippy::all)]
    pub fn set_html_for(&self, html_for: &str) {
        #[cfg(all(feature = "HtmlLabelElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_html_for_HTMLLabelElement(
                self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                html_for: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_html_for_HTMLLabelElement(
            self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            html_for: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(html_for);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let html_for = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(html_for);
                __widl_f_set_html_for_HTMLLabelElement(self_, html_for)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlLabelElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_control_HTMLLabelElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlLabelElement as WasmDescribe>::describe();
    <Option<HtmlElement> as WasmDescribe>::describe();
}
impl HtmlLabelElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlLabelElement",))]
    #[allow(bad_style)]
    #[doc = "The `control` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlLabelElement`*"]
    #[allow(clippy::all)]
    pub fn control(&self) -> Option<HtmlElement> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlLabelElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_control_HTMLLabelElement(
                self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_control_HTMLLabelElement(
            self_: <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlLabelElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_control_HTMLLabelElement(self_)
            };
            <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_fa85e67b24fc772d: [u8; 507usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB9\x01\0\0\0\0\x05\0\0\x02\x10HTMLLabelElement\"__widl_instanceof_HTMLLabelElement\0\0\0\0\x1E__widl_f_form_HTMLLabelElement\0\0\0\x01\x10HTMLLabelElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0\"__widl_f_html_for_HTMLLabelElement\0\0\0\x01\x10HTMLLabelElement\x01\0\x01\x07htmlFor\x01\x01\x05self_\x07htmlFor\0\0\0&__widl_f_set_html_for_HTMLLabelElement\0\0\0\x01\x10HTMLLabelElement\x01\0\x02\x07htmlFor\x01\x02\x05self_\x08html_for\x07htmlFor\0\0\0!__widl_f_control_HTMLLabelElement\0\0\0\x01\x10HTMLLabelElement\x01\0\x01\x07control\x01\x01\x05self_\x07control\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
