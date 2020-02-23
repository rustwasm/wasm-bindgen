use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLBaseElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlBaseElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlBaseElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlBaseElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(66u32);
            inform(97u32);
            inform(115u32);
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
    impl core::ops::Deref for HtmlBaseElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlBaseElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlBaseElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlBaseElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlBaseElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlBaseElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlBaseElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlBaseElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlBaseElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlBaseElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlBaseElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlBaseElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlBaseElement {
            HtmlBaseElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlBaseElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlBaseElement> for HtmlBaseElement {
        #[inline]
        fn as_ref(&self) -> &HtmlBaseElement {
            self
        }
    }
    impl From<HtmlBaseElement> for JsValue {
        #[inline]
        fn from(obj: HtmlBaseElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlBaseElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLBaseElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLBaseElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLBaseElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlBaseElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlBaseElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlBaseElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlBaseElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlBaseElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBaseElement> for Element {
    #[inline]
    fn from(obj: HtmlBaseElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlBaseElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBaseElement> for Node {
    #[inline]
    fn from(obj: HtmlBaseElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlBaseElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBaseElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlBaseElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlBaseElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlBaseElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlBaseElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlBaseElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlBaseElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_HTMLBaseElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBaseElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBaseElement {
    #[cfg(all(feature = "HtmlBaseElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "HtmlBaseElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_HTMLBaseElement(
                self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_HTMLBaseElement(
            self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_HTMLBaseElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBaseElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_href_HTMLBaseElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBaseElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBaseElement {
    #[cfg(all(feature = "HtmlBaseElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    #[allow(clippy::all)]
    pub fn set_href(&self, href: &str) {
        #[cfg(all(feature = "HtmlBaseElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_href_HTMLBaseElement(
                self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_href_HTMLBaseElement(
            self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let href = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(href);
                __widl_f_set_href_HTMLBaseElement(self_, href)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlBaseElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_HTMLBaseElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlBaseElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlBaseElement {
    #[cfg(all(feature = "HtmlBaseElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> String {
        #[cfg(all(feature = "HtmlBaseElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_HTMLBaseElement(
                self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_HTMLBaseElement(
            self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_HTMLBaseElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlBaseElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_HTMLBaseElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlBaseElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlBaseElement {
    #[cfg(all(feature = "HtmlBaseElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)\n\n*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
    #[allow(clippy::all)]
    pub fn set_target(&self, target: &str) {
        #[cfg(all(feature = "HtmlBaseElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_HTMLBaseElement(
                self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_HTMLBaseElement(
            self_: <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlBaseElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_set_target_HTMLBaseElement(self_, target)
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
pub static __WASM_BINDGEN_GENERATED_e9fcb8c165ddf89a: [u8; 487usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xA5\x01\0\0\0\0\x05\0\0\x02\x0FHTMLBaseElement!__widl_instanceof_HTMLBaseElement\0\0\0\0\x1D__widl_f_href_HTMLBaseElement\0\0\0\x01\x0FHTMLBaseElement\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0!__widl_f_set_href_HTMLBaseElement\0\0\0\x01\x0FHTMLBaseElement\x01\0\x02\x04href\x01\x02\x05self_\x04href\x04href\0\0\0\x1F__widl_f_target_HTMLBaseElement\0\0\0\x01\x0FHTMLBaseElement\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0#__widl_f_set_target_HTMLBaseElement\0\0\0\x01\x0FHTMLBaseElement\x01\0\x02\x06target\x01\x02\x05self_\x06target\x06target\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
