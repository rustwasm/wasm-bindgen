use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLFieldSetElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlFieldSetElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlFieldSetElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlFieldSetElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(70u32);
            inform(105u32);
            inform(101u32);
            inform(108u32);
            inform(100u32);
            inform(83u32);
            inform(101u32);
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
    impl core::ops::Deref for HtmlFieldSetElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlFieldSetElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlFieldSetElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlFieldSetElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlFieldSetElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlFieldSetElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlFieldSetElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlFieldSetElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlFieldSetElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlFieldSetElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlFieldSetElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlFieldSetElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlFieldSetElement {
            HtmlFieldSetElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlFieldSetElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlFieldSetElement> for HtmlFieldSetElement {
        #[inline]
        fn as_ref(&self) -> &HtmlFieldSetElement {
            self
        }
    }
    impl From<HtmlFieldSetElement> for JsValue {
        #[inline]
        fn from(obj: HtmlFieldSetElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlFieldSetElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLFieldSetElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLFieldSetElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLFieldSetElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlFieldSetElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlFieldSetElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlFieldSetElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlFieldSetElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlFieldSetElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFieldSetElement> for Element {
    #[inline]
    fn from(obj: HtmlFieldSetElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlFieldSetElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFieldSetElement> for Node {
    #[inline]
    fn from(obj: HtmlFieldSetElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlFieldSetElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFieldSetElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlFieldSetElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlFieldSetElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFieldSetElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlFieldSetElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlFieldSetElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLFieldSetElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLFieldSetElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLFieldSetElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLFieldSetElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLFieldSetElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement", feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement", feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFieldSetElement", feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLFieldSetElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLFieldSetElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLFieldSetElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/type)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLFieldSetElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_elements_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `elements` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/elements)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn elements(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_elements_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_elements_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_elements_HTMLFieldSetElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLFieldSetElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlFieldSetElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLFieldSetElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFieldSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLFieldSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFieldSetElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFieldSetElement {
    #[cfg(all(feature = "HtmlFieldSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlFieldSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLFieldSetElement(
                self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLFieldSetElement(
            self_: <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFieldSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLFieldSetElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_bdd7637b10c1557e: [u8; 1427usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}Q\x05\0\0\0\0\x0E\0\0\x02\x13HTMLFieldSetElement%__widl_instanceof_HTMLFieldSetElement\0\0\0\0+__widl_f_check_validity_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0,__widl_f_report_validity_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\00__widl_f_set_custom_validity_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0%__widl_f_disabled_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0)__widl_f_set_disabled_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0!__widl_f_form_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0!__widl_f_name_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0%__widl_f_set_name_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0!__widl_f_type_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0%__widl_f_elements_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x08elements\x01\x01\x05self_\x08elements\0\0\0*__widl_f_will_validate_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0%__widl_f_validity_HTMLFieldSetElement\0\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0/__widl_f_validation_message_HTMLFieldSetElement\x01\0\0\x01\x13HTMLFieldSetElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
