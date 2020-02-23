use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLOutputElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlOutputElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlOutputElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlOutputElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(79u32);
            inform(117u32);
            inform(116u32);
            inform(112u32);
            inform(117u32);
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
    impl core::ops::Deref for HtmlOutputElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlOutputElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlOutputElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlOutputElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlOutputElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlOutputElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlOutputElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlOutputElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlOutputElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlOutputElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlOutputElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlOutputElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlOutputElement {
            HtmlOutputElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlOutputElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlOutputElement> for HtmlOutputElement {
        #[inline]
        fn as_ref(&self) -> &HtmlOutputElement {
            self
        }
    }
    impl From<HtmlOutputElement> for JsValue {
        #[inline]
        fn from(obj: HtmlOutputElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlOutputElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLOutputElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLOutputElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLOutputElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlOutputElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlOutputElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlOutputElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlOutputElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlOutputElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOutputElement> for Element {
    #[inline]
    fn from(obj: HtmlOutputElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlOutputElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOutputElement> for Node {
    #[inline]
    fn from(obj: HtmlOutputElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlOutputElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOutputElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlOutputElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlOutputElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOutputElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlOutputElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlOutputElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLOutputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLOutputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(error);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLOutputElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_html_for_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "DomTokenList", feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `htmlFor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/htmlFor)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn html_for(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_html_for_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_html_for_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_html_for_HTMLOutputElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLOutputElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLOutputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLOutputElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/type)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLOutputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_value_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn default_value(&self) -> String {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_value_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_value_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_value_HTMLOutputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_value_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_value(&self, default_value: &str) {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_value_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_value_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default_value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_value =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_value);
                __widl_f_set_default_value_HTMLOutputElement(self_, default_value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLOutputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLOutputElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLOutputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlOutputElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLOutputElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOutputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLOutputElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlOutputElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLOutputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOutputElement as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl HtmlOutputElement {
    #[cfg(all(feature = "HtmlOutputElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlOutputElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> NodeList {
        #[cfg(all(feature = "HtmlOutputElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLOutputElement(
                self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLOutputElement(
            self_: <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOutputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLOutputElement(self_)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e81d76bc1389d964: [u8; 1656usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}6\x06\0\0\0\0\x11\0\0\x02\x11HTMLOutputElement#__widl_instanceof_HTMLOutputElement\0\0\0\0)__widl_f_check_validity_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0*__widl_f_report_validity_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0.__widl_f_set_custom_validity_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0#__widl_f_html_for_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x07htmlFor\x01\x01\x05self_\x07htmlFor\0\0\0\x1F__widl_f_form_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0\x1F__widl_f_name_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_set_name_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\x1F__widl_f_type_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0(__widl_f_default_value_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x0CdefaultValue\x01\x01\x05self_\x0CdefaultValue\0\0\0,__widl_f_set_default_value_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x02\x0CdefaultValue\x01\x02\x05self_\rdefault_value\x0CdefaultValue\0\0\0 __widl_f_value_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0$__widl_f_set_value_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0(__widl_f_will_validate_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0#__widl_f_validity_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0-__widl_f_validation_message_HTMLOutputElement\x01\0\0\x01\x11HTMLOutputElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0!__widl_f_labels_HTMLOutputElement\0\0\0\x01\x11HTMLOutputElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
