use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLButtonElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlButtonElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlButtonElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlButtonElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(66u32);
            inform(117u32);
            inform(116u32);
            inform(116u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlButtonElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlButtonElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlButtonElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlButtonElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlButtonElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlButtonElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlButtonElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlButtonElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlButtonElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlButtonElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlButtonElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlButtonElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlButtonElement {
            HtmlButtonElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlButtonElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlButtonElement> for HtmlButtonElement {
        #[inline]
        fn as_ref(&self) -> &HtmlButtonElement {
            self
        }
    }
    impl From<HtmlButtonElement> for JsValue {
        #[inline]
        fn from(obj: HtmlButtonElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlButtonElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLButtonElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLButtonElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLButtonElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlButtonElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlButtonElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlButtonElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlButtonElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlButtonElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlButtonElement> for Element {
    #[inline]
    fn from(obj: HtmlButtonElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlButtonElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlButtonElement> for Node {
    #[inline]
    fn from(obj: HtmlButtonElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlButtonElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlButtonElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlButtonElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlButtonElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlButtonElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlButtonElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlButtonElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLButtonElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLButtonElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLButtonElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autofocus_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn autofocus(&self) -> bool {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autofocus_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autofocus_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autofocus_HTMLButtonElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autofocus_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_autofocus(&self, autofocus: bool) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autofocus_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autofocus: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autofocus_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            autofocus: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(autofocus);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autofocus = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autofocus);
                __widl_f_set_autofocus_HTMLButtonElement(self_, autofocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLButtonElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLButtonElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement", feature = "HtmlFormElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement", feature = "HtmlFormElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`, `HtmlFormElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlButtonElement", feature = "HtmlFormElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLButtonElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_action_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formAction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn form_action(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_action_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_action_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_action_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_action_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formAction` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_action(&self, form_action: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_action_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_action_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            form_action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(form_action);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_action =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_action);
                __widl_f_set_form_action_HTMLButtonElement(self_, form_action)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_enctype_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formEnctype` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn form_enctype(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_enctype_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_enctype_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_enctype_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_enctype_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formEnctype` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_enctype(&self, form_enctype: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_enctype_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_enctype: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_enctype_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            form_enctype: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(form_enctype);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_enctype =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_enctype);
                __widl_f_set_form_enctype_HTMLButtonElement(self_, form_enctype)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_method_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formMethod` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn form_method(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_method_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_method_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_method_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_method_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formMethod` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_method(&self, form_method: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_method_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_method_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            form_method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(form_method);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_method =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_method);
                __widl_f_set_form_method_HTMLButtonElement(self_, form_method)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_no_validate_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formNoValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn form_no_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_no_validate_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_no_validate_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_no_validate_HTMLButtonElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_no_validate_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formNoValidate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_no_validate(&self, form_no_validate: bool) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_no_validate_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_no_validate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_no_validate_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            form_no_validate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(form_no_validate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_no_validate =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_no_validate);
                __widl_f_set_form_no_validate_HTMLButtonElement(self_, form_no_validate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_target_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formTarget` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn form_target(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_target_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_target_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_target_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_target_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `formTarget` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_target(&self, form_target: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_target_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_target_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            form_target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(form_target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_target =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_target);
                __widl_f_set_form_target_HTMLButtonElement(self_, form_target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLButtonElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLButtonElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLButtonElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLButtonElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLButtonElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlButtonElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLButtonElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlButtonElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLButtonElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlButtonElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLButtonElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlButtonElement as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl HtmlButtonElement {
    #[cfg(all(feature = "HtmlButtonElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlButtonElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> NodeList {
        #[cfg(all(feature = "HtmlButtonElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLButtonElement(
                self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLButtonElement(
            self_: <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlButtonElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLButtonElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_f0c9777b0ee01fd4: [u8; 2913usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1F\x0B\0\0\0\0\x1D\0\0\x02\x11HTMLButtonElement#__widl_instanceof_HTMLButtonElement\0\0\0\0)__widl_f_check_validity_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0*__widl_f_report_validity_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0.__widl_f_set_custom_validity_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0$__widl_f_autofocus_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\tautofocus\x01\x01\x05self_\tautofocus\0\0\0(__widl_f_set_autofocus_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\tautofocus\x01\x02\x05self_\tautofocus\tautofocus\0\0\0#__widl_f_disabled_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0'__widl_f_set_disabled_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0\x1F__widl_f_form_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0&__widl_f_form_action_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\nformAction\x01\x01\x05self_\nformAction\0\0\0*__widl_f_set_form_action_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\nformAction\x01\x02\x05self_\x0Bform_action\nformAction\0\0\0'__widl_f_form_enctype_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x0BformEnctype\x01\x01\x05self_\x0BformEnctype\0\0\0+__widl_f_set_form_enctype_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\x0BformEnctype\x01\x02\x05self_\x0Cform_enctype\x0BformEnctype\0\0\0&__widl_f_form_method_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\nformMethod\x01\x01\x05self_\nformMethod\0\0\0*__widl_f_set_form_method_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\nformMethod\x01\x02\x05self_\x0Bform_method\nformMethod\0\0\0+__widl_f_form_no_validate_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x0EformNoValidate\x01\x01\x05self_\x0EformNoValidate\0\0\0/__widl_f_set_form_no_validate_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\x0EformNoValidate\x01\x02\x05self_\x10form_no_validate\x0EformNoValidate\0\0\0&__widl_f_form_target_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\nformTarget\x01\x01\x05self_\nformTarget\0\0\0*__widl_f_set_form_target_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\nformTarget\x01\x02\x05self_\x0Bform_target\nformTarget\0\0\0\x1F__widl_f_name_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_set_name_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\x1F__widl_f_type_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0#__widl_f_set_type_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0 __widl_f_value_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0$__widl_f_set_value_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0(__widl_f_will_validate_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0#__widl_f_validity_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0-__widl_f_validation_message_HTMLButtonElement\x01\0\0\x01\x11HTMLButtonElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0!__widl_f_labels_HTMLButtonElement\0\0\0\x01\x11HTMLButtonElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
