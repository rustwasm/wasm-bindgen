use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTextAreaElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTextAreaElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTextAreaElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTextAreaElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(65u32);
            inform(114u32);
            inform(101u32);
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
    impl core::ops::Deref for HtmlTextAreaElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTextAreaElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTextAreaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTextAreaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTextAreaElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTextAreaElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTextAreaElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTextAreaElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTextAreaElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTextAreaElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTextAreaElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTextAreaElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTextAreaElement {
            HtmlTextAreaElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTextAreaElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTextAreaElement> for HtmlTextAreaElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTextAreaElement {
            self
        }
    }
    impl From<HtmlTextAreaElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTextAreaElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTextAreaElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTextAreaElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTextAreaElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTextAreaElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTextAreaElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTextAreaElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTextAreaElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTextAreaElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTextAreaElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTextAreaElement> for Element {
    #[inline]
    fn from(obj: HtmlTextAreaElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTextAreaElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTextAreaElement> for Node {
    #[inline]
    fn from(obj: HtmlTextAreaElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTextAreaElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTextAreaElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTextAreaElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTextAreaElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTextAreaElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTextAreaElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTextAreaElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_select_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `select()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/select)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn select(&self) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_select_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_select_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_select_HTMLTextAreaElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLTextAreaElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_range_text_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `setRangeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_range_text(&self, replacement: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_range_text_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                replacement: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_range_text_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            replacement: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(replacement);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let replacement =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(replacement);
                __widl_f_set_range_text_HTMLTextAreaElement(self_, replacement)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_range_text_with_start_and_end_HTMLTextAreaElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `setRangeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_range_text_with_start_and_end(
        &self,
        replacement: &str,
        start: u32,
        end: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_range_text_with_start_and_end_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                replacement: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_range_text_with_start_and_end_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            replacement: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(replacement);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let replacement =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(replacement);
                let start = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_set_range_text_with_start_and_end_HTMLTextAreaElement(
                    self_,
                    replacement,
                    start,
                    end,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_range_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `setSelectionRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_range(&self, start: u32, end: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_range_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_range_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_set_selection_range_HTMLTextAreaElement(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_range_with_direction_HTMLTextAreaElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `setSelectionRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_range_with_direction(
        &self,
        start: u32,
        end: u32,
        direction: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_range_with_direction_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_range_with_direction_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            direction: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start);
            drop(end);
            drop(direction);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                let direction = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                __widl_f_set_selection_range_with_direction_HTMLTextAreaElement(
                    self_, start, end, direction,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autocomplete_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn autocomplete(&self) -> String {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autocomplete_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autocomplete_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autocomplete_HTMLTextAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autocomplete_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_autocomplete(&self, autocomplete: &str) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autocomplete_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autocomplete: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autocomplete_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autocomplete =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autocomplete);
                __widl_f_set_autocomplete_HTMLTextAreaElement(self_, autocomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autofocus_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn autofocus(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autofocus_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autofocus_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autofocus_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autofocus_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_autofocus(&self, autofocus: bool) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autofocus_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autofocus: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autofocus_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autofocus = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autofocus);
                __widl_f_set_autofocus_HTMLTextAreaElement(self_, autofocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cols_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `cols` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn cols(&self) -> u32 {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cols_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cols_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cols_HTMLTextAreaElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cols_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `cols` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_cols(&self, cols: u32) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cols_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cols: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cols_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cols: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cols);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cols = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cols);
                __widl_f_set_cols_HTMLTextAreaElement(self_, cols)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLTextAreaElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLTextAreaElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_length_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `maxLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn max_length(&self) -> i32 {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_length_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_length_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_length_HTMLTextAreaElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_length_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `maxLength` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_max_length(&self, max_length: i32) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_length_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_length: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_length_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_length: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max_length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_length = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_length);
                __widl_f_set_max_length_HTMLTextAreaElement(self_, max_length)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_length_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `minLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn min_length(&self) -> i32 {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_length_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_length_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_length_HTMLTextAreaElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_min_length_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `minLength` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_min_length(&self, min_length: i32) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_min_length_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                min_length: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_min_length_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            min_length: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(min_length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let min_length = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(min_length);
                __widl_f_set_min_length_HTMLTextAreaElement(self_, min_length)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLTextAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLTextAreaElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_placeholder_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `placeholder` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn placeholder(&self) -> String {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_placeholder_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_placeholder_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_placeholder_HTMLTextAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_placeholder_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `placeholder` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_placeholder(&self, placeholder: &str) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_placeholder_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                placeholder: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_placeholder_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            placeholder: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(placeholder);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let placeholder =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(placeholder);
                __widl_f_set_placeholder_HTMLTextAreaElement(self_, placeholder)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_only_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `readOnly` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn read_only(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_only_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_only_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_read_only_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_read_only_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `readOnly` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_read_only(&self, read_only: bool) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_read_only_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                read_only: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_read_only_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            read_only: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(read_only);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let read_only = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(read_only);
                __widl_f_set_read_only_HTMLTextAreaElement(self_, read_only)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `required` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn required(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_required_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `required` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_required(&self, required: bool) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_required_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                required: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_required_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            required: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(required);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let required = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(required);
                __widl_f_set_required_HTMLTextAreaElement(self_, required)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rows_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `rows` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn rows(&self) -> u32 {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rows_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rows_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rows_HTMLTextAreaElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rows_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `rows` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_rows(&self, rows: u32) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rows_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rows: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rows_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rows: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rows);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rows = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rows);
                __widl_f_set_rows_HTMLTextAreaElement(self_, rows)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_wrap_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `wrap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn wrap(&self) -> String {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_wrap_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_wrap_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_wrap_HTMLTextAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_wrap_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `wrap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_wrap(&self, wrap: &str) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_wrap_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                wrap: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_wrap_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            wrap: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(wrap);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let wrap = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(wrap);
                __widl_f_set_wrap_HTMLTextAreaElement(self_, wrap)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/type)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLTextAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_value_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn default_value(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_value_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_value_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_value_HTMLTextAreaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_value_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_value(&self, default_value: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_value_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_value_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_value =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_value);
                __widl_f_set_default_value_HTMLTextAreaElement(self_, default_value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLTextAreaElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLTextAreaElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_length_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `textLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/textLength)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn text_length(&self) -> u32 {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_length_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_length_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_length_HTMLTextAreaElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLTextAreaElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlTextAreaElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLTextAreaElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLTextAreaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> NodeList {
        #[cfg(all(feature = "HtmlTextAreaElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLTextAreaElement(self_)
            };
            <NodeList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_start_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn selection_start(&self) -> Result<Option<u32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_start_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_start_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_start_HTMLTextAreaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<u32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_start_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionStart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_start(
        &self,
        selection_start: Option<u32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_start_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selection_start: <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_start_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selection_start: <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selection_start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selection_start =
                    <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selection_start);
                __widl_f_set_selection_start_HTMLTextAreaElement(self_, selection_start)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_end_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn selection_end(&self) -> Result<Option<u32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_end_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_end_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_end_HTMLTextAreaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<u32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_end_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionEnd` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_end(
        &self,
        selection_end: Option<u32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_end_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selection_end: <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_end_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selection_end: <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selection_end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selection_end =
                    <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selection_end);
                __widl_f_set_selection_end_HTMLTextAreaElement(self_, selection_end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_direction_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionDirection` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn selection_direction(&self) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_direction_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_direction_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_direction_HTMLTextAreaElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlTextAreaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_direction_HTMLTextAreaElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTextAreaElement as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTextAreaElement {
    #[cfg(all(feature = "HtmlTextAreaElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionDirection` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_direction(
        &self,
        selection_direction: Option<&str>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlTextAreaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_direction_HTMLTextAreaElement(
                self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selection_direction: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_direction_HTMLTextAreaElement(
            self_: <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selection_direction: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selection_direction);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTextAreaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selection_direction =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        selection_direction,
                    );
                __widl_f_set_selection_direction_HTMLTextAreaElement(self_, selection_direction)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e4cc95fdcce8fc99: [u8; 5270usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}T\x14\0\0\0\02\0\0\x02\x13HTMLTextAreaElement%__widl_instanceof_HTMLTextAreaElement\0\0\0\0+__widl_f_check_validity_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0,__widl_f_report_validity_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0#__widl_f_select_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x01\x05self_\x06select\0\0\00__widl_f_set_custom_validity_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0+__widl_f_set_range_text_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x02\x05self_\x0Breplacement\x0CsetRangeText\0\0\0>__widl_f_set_range_text_with_start_and_end_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x04\x05self_\x0Breplacement\x05start\x03end\x0CsetRangeText\0\0\00__widl_f_set_selection_range_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x03\x05self_\x05start\x03end\x11setSelectionRange\0\0\0?__widl_f_set_selection_range_with_direction_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\0\x01\x04\x05self_\x05start\x03end\tdirection\x11setSelectionRange\0\0\0)__widl_f_autocomplete_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x0Cautocomplete\x01\x01\x05self_\x0Cautocomplete\0\0\0-__widl_f_set_autocomplete_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x0Cautocomplete\x01\x02\x05self_\x0Cautocomplete\x0Cautocomplete\0\0\0&__widl_f_autofocus_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\tautofocus\x01\x01\x05self_\tautofocus\0\0\0*__widl_f_set_autofocus_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\tautofocus\x01\x02\x05self_\tautofocus\tautofocus\0\0\0!__widl_f_cols_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x04cols\x01\x01\x05self_\x04cols\0\0\0%__widl_f_set_cols_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x04cols\x01\x02\x05self_\x04cols\x04cols\0\0\0%__widl_f_disabled_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0)__widl_f_set_disabled_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0!__widl_f_form_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0'__widl_f_max_length_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\tmaxLength\x01\x01\x05self_\tmaxLength\0\0\0+__widl_f_set_max_length_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\tmaxLength\x01\x02\x05self_\nmax_length\tmaxLength\0\0\0'__widl_f_min_length_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\tminLength\x01\x01\x05self_\tminLength\0\0\0+__widl_f_set_min_length_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\tminLength\x01\x02\x05self_\nmin_length\tminLength\0\0\0!__widl_f_name_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0%__widl_f_set_name_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0(__widl_f_placeholder_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x0Bplaceholder\x01\x01\x05self_\x0Bplaceholder\0\0\0,__widl_f_set_placeholder_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x0Bplaceholder\x01\x02\x05self_\x0Bplaceholder\x0Bplaceholder\0\0\0&__widl_f_read_only_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x08readOnly\x01\x01\x05self_\x08readOnly\0\0\0*__widl_f_set_read_only_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x08readOnly\x01\x02\x05self_\tread_only\x08readOnly\0\0\0%__widl_f_required_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x08required\x01\x01\x05self_\x08required\0\0\0)__widl_f_set_required_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x08required\x01\x02\x05self_\x08required\x08required\0\0\0!__widl_f_rows_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x04rows\x01\x01\x05self_\x04rows\0\0\0%__widl_f_set_rows_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x04rows\x01\x02\x05self_\x04rows\x04rows\0\0\0!__widl_f_wrap_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x04wrap\x01\x01\x05self_\x04wrap\0\0\0%__widl_f_set_wrap_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x04wrap\x01\x02\x05self_\x04wrap\x04wrap\0\0\0!__widl_f_type_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0*__widl_f_default_value_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x0CdefaultValue\x01\x01\x05self_\x0CdefaultValue\0\0\0.__widl_f_set_default_value_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x0CdefaultValue\x01\x02\x05self_\rdefault_value\x0CdefaultValue\0\0\0\"__widl_f_value_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0&__widl_f_set_value_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0(__widl_f_text_length_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\ntextLength\x01\x01\x05self_\ntextLength\0\0\0*__widl_f_will_validate_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0%__widl_f_validity_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0/__widl_f_validation_message_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0#__widl_f_labels_HTMLTextAreaElement\0\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0,__widl_f_selection_start_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x0EselectionStart\x01\x01\x05self_\x0EselectionStart\0\0\00__widl_f_set_selection_start_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x0EselectionStart\x01\x02\x05self_\x0Fselection_start\x0EselectionStart\0\0\0*__widl_f_selection_end_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x0CselectionEnd\x01\x01\x05self_\x0CselectionEnd\0\0\0.__widl_f_set_selection_end_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x0CselectionEnd\x01\x02\x05self_\rselection_end\x0CselectionEnd\0\0\00__widl_f_selection_direction_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x01\x12selectionDirection\x01\x01\x05self_\x12selectionDirection\0\0\04__widl_f_set_selection_direction_HTMLTextAreaElement\x01\0\0\x01\x13HTMLTextAreaElement\x01\0\x02\x12selectionDirection\x01\x02\x05self_\x13selection_direction\x12selectionDirection\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
