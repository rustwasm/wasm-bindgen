use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLFormElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlFormElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlFormElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlFormElement {
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
            inform(114u32);
            inform(109u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlFormElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlFormElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlFormElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlFormElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlFormElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlFormElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlFormElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlFormElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlFormElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlFormElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlFormElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlFormElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlFormElement {
            HtmlFormElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlFormElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlFormElement> for HtmlFormElement {
        #[inline]
        fn as_ref(&self) -> &HtmlFormElement {
            self
        }
    }
    impl From<HtmlFormElement> for JsValue {
        #[inline]
        fn from(obj: HtmlFormElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlFormElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLFormElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLFormElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLFormElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlFormElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlFormElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlFormElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlFormElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlFormElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFormElement> for Element {
    #[inline]
    fn from(obj: HtmlFormElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlFormElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFormElement> for Node {
    #[inline]
    fn from(obj: HtmlFormElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlFormElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFormElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlFormElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlFormElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFormElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlFormElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlFormElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLFormElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLFormElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reset_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `reset()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/reset)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn reset(&self) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reset_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reset_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reset_HTMLFormElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_submit_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `submit()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn submit(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_submit_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_submit_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_submit_HTMLFormElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_index_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "Element", feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Element`, `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn get_with_index(&self, index: u32) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_index_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_index_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_with_index_HTMLFormElement(self_, index)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_with_name_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn get_with_name(&self, name: &str) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_with_name_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_with_name_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_get_with_name_HTMLFormElement(self_, name)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_accept_charset_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `acceptCharset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn accept_charset(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_accept_charset_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_accept_charset_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_accept_charset_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_accept_charset_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `acceptCharset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/acceptCharset)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_accept_charset(&self, accept_charset: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_accept_charset_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                accept_charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_accept_charset_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            accept_charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(accept_charset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let accept_charset =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(accept_charset);
                __widl_f_set_accept_charset_HTMLFormElement(self_, accept_charset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_action_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `action` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn action(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_action_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_action_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_action_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_action_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `action` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/action)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_action(&self, action: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_action_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_action_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(action);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let action = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(action);
                __widl_f_set_action_HTMLFormElement(self_, action)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autocomplete_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn autocomplete(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autocomplete_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autocomplete_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autocomplete_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autocomplete_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_autocomplete(&self, autocomplete: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autocomplete_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autocomplete: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autocomplete_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            autocomplete: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(autocomplete);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autocomplete =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autocomplete);
                __widl_f_set_autocomplete_HTMLFormElement(self_, autocomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_enctype_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `enctype` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn enctype(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_enctype_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_enctype_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_enctype_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_enctype_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `enctype` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/enctype)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_enctype(&self, enctype: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_enctype_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                enctype: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_enctype_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            enctype: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(enctype);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let enctype = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(enctype);
                __widl_f_set_enctype_HTMLFormElement(self_, enctype)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_encoding_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `encoding` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn encoding(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_encoding_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_encoding_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_encoding_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_encoding_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `encoding` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/encoding)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_encoding(&self, encoding: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_encoding_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_encoding_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoding: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(encoding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let encoding = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(encoding);
                __widl_f_set_encoding_HTMLFormElement(self_, encoding)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_method_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `method` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn method(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_method_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_method_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_method_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_method_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `method` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/method)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_method(&self, method: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_method_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_method_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(method);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let method = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(method);
                __widl_f_set_method_HTMLFormElement(self_, method)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLFormElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_validate_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `noValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn no_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_validate_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_validate_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_validate_HTMLFormElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_no_validate_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `noValidate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/noValidate)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_no_validate(&self, no_validate: bool) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_no_validate_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                no_validate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_no_validate_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            no_validate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(no_validate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let no_validate =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(no_validate);
                __widl_f_set_no_validate_HTMLFormElement(self_, no_validate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> String {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_HTMLFormElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/target)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn set_target(&self, target: &str) {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_set_target_HTMLFormElement(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elements_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `elements` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/elements)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn elements(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elements_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elements_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_elements_HTMLFormElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_HTMLFormElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFormElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlFormElement {
    #[cfg(all(feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/length)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> i32 {
        #[cfg(all(feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_HTMLFormElement(
                self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_HTMLFormElement(
            self_: <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFormElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_HTMLFormElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7a5c5383ce843b88: [u8; 2484usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}r\t\0\0\0\0\x1B\0\0\x02\x0FHTMLFormElement!__widl_instanceof_HTMLFormElement\0\0\0\0'__widl_f_check_validity_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0(__widl_f_report_validity_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0\x1E__widl_f_reset_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\0\x01\x01\x05self_\x05reset\0\0\0\x1F__widl_f_submit_HTMLFormElement\x01\0\0\x01\x0FHTMLFormElement\x01\0\0\x01\x01\x05self_\x06submit\0\0\0'__widl_f_get_with_index_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0&__widl_f_get_with_name_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x03\x01\x02\x05self_\x04name\x03get\0\0\0'__widl_f_accept_charset_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\racceptCharset\x01\x01\x05self_\racceptCharset\0\0\0+__widl_f_set_accept_charset_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\racceptCharset\x01\x02\x05self_\x0Eaccept_charset\racceptCharset\0\0\0\x1F__widl_f_action_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x06action\x01\x01\x05self_\x06action\0\0\0#__widl_f_set_action_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x06action\x01\x02\x05self_\x06action\x06action\0\0\0%__widl_f_autocomplete_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x0Cautocomplete\x01\x01\x05self_\x0Cautocomplete\0\0\0)__widl_f_set_autocomplete_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x0Cautocomplete\x01\x02\x05self_\x0Cautocomplete\x0Cautocomplete\0\0\0 __widl_f_enctype_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x07enctype\x01\x01\x05self_\x07enctype\0\0\0$__widl_f_set_enctype_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x07enctype\x01\x02\x05self_\x07enctype\x07enctype\0\0\0!__widl_f_encoding_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x08encoding\x01\x01\x05self_\x08encoding\0\0\0%__widl_f_set_encoding_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x08encoding\x01\x02\x05self_\x08encoding\x08encoding\0\0\0\x1F__widl_f_method_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x06method\x01\x01\x05self_\x06method\0\0\0#__widl_f_set_method_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x06method\x01\x02\x05self_\x06method\x06method\0\0\0\x1D__widl_f_name_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0!__widl_f_set_name_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0$__widl_f_no_validate_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\nnoValidate\x01\x01\x05self_\nnoValidate\0\0\0(__widl_f_set_no_validate_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\nnoValidate\x01\x02\x05self_\x0Bno_validate\nnoValidate\0\0\0\x1F__widl_f_target_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0#__widl_f_set_target_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x02\x06target\x01\x02\x05self_\x06target\x06target\0\0\0!__widl_f_elements_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x08elements\x01\x01\x05self_\x08elements\0\0\0\x1F__widl_f_length_HTMLFormElement\0\0\0\x01\x0FHTMLFormElement\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
