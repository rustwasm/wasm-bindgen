use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLInputElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlInputElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlInputElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlInputElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(73u32);
            inform(110u32);
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
    impl core::ops::Deref for HtmlInputElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlInputElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlInputElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlInputElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlInputElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlInputElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlInputElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlInputElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlInputElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlInputElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlInputElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlInputElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlInputElement {
            HtmlInputElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlInputElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlInputElement> for HtmlInputElement {
        #[inline]
        fn as_ref(&self) -> &HtmlInputElement {
            self
        }
    }
    impl From<HtmlInputElement> for JsValue {
        #[inline]
        fn from(obj: HtmlInputElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlInputElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLInputElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLInputElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLInputElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlInputElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlInputElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlInputElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlInputElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlInputElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlInputElement> for Element {
    #[inline]
    fn from(obj: HtmlInputElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlInputElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlInputElement> for Node {
    #[inline]
    fn from(obj: HtmlInputElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlInputElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlInputElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlInputElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlInputElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlInputElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlInputElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlInputElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_select_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `select()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn select(&self) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_select_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_select_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_select_HTMLInputElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLInputElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_range_text_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `setRangeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_range_text(&self, replacement: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_range_text_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                replacement: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_range_text_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let replacement =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(replacement);
                __widl_f_set_range_text_HTMLInputElement(self_, replacement)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_range_text_with_start_and_end_HTMLInputElement()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `setRangeText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setRangeText)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_range_text_with_start_and_end(
        &self,
        replacement: &str,
        start: u32,
        end: u32,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_range_text_with_start_and_end_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                replacement: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_range_text_with_start_and_end_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let replacement =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(replacement);
                let start = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_set_range_text_with_start_and_end_HTMLInputElement(
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
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_range_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `setSelectionRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_range(&self, start: u32, end: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_range_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_range_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_set_selection_range_HTMLInputElement(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_range_with_direction_HTMLInputElement()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `setSelectionRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/setSelectionRange)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_range_with_direction(
        &self,
        start: u32,
        end: u32,
        direction: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_range_with_direction_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                direction: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_range_with_direction_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                let direction = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(direction);
                __widl_f_set_selection_range_with_direction_HTMLInputElement(
                    self_, start, end, direction,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_accept_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `accept` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn accept(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_accept_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_accept_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_accept_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_accept_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `accept` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/accept)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_accept(&self, accept: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_accept_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                accept: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_accept_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            accept: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(accept);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let accept = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(accept);
                __widl_f_set_accept_HTMLInputElement(self_, accept)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_alt_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `alt` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn alt(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_alt_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_alt_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_alt_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_alt_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `alt` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/alt)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_alt(&self, alt: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_alt_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                alt: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_alt_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            alt: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(alt);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let alt = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(alt);
                __widl_f_set_alt_HTMLInputElement(self_, alt)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autocomplete_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn autocomplete(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autocomplete_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autocomplete_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autocomplete_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autocomplete_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_autocomplete(&self, autocomplete: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autocomplete_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autocomplete: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autocomplete_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autocomplete =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autocomplete);
                __widl_f_set_autocomplete_HTMLInputElement(self_, autocomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autofocus_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn autofocus(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autofocus_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autofocus_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autofocus_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autofocus_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_autofocus(&self, autofocus: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autofocus_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autofocus: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autofocus_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autofocus = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autofocus);
                __widl_f_set_autofocus_HTMLInputElement(self_, autofocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_checked_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultChecked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn default_checked(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_checked_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_checked_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_checked_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_checked_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultChecked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_checked(&self, default_checked: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_checked_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_checked_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default_checked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_checked =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_checked);
                __widl_f_set_default_checked_HTMLInputElement(self_, default_checked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_checked_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `checked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn checked(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_checked_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_checked_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_checked_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_checked_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `checked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_checked(&self, checked: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_checked_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_checked_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(checked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let checked = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(checked);
                __widl_f_set_checked_HTMLInputElement(self_, checked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLInputElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLInputElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileList", feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_files_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<FileList> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "FileList", feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `files` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)\n\n*This API requires the following crate features to be activated: `FileList`, `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn files(&self) -> Option<FileList> {
        #[cfg(all(feature = "FileList", feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_files_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<FileList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_files_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<FileList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_files_HTMLInputElement(self_)
            };
            <Option<FileList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "FileList", feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_files_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<&FileList> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "FileList", feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `files` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/files)\n\n*This API requires the following crate features to be activated: `FileList`, `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_files(&self, files: Option<&FileList>) {
        #[cfg(all(feature = "FileList", feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_files_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                files: <Option<&FileList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_files_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            files: <Option<&FileList> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(files);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let files =
                    <Option<&FileList> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(files);
                __widl_f_set_files_HTMLInputElement(self_, files)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_action_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formAction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn form_action(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_action_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_action_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_action_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_action_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formAction` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formAction)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_action(&self, form_action: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_action_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_action: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_action_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_action =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_action);
                __widl_f_set_form_action_HTMLInputElement(self_, form_action)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_enctype_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formEnctype` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn form_enctype(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_enctype_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_enctype_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_enctype_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_enctype_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formEnctype` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formEnctype)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_enctype(&self, form_enctype: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_enctype_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_enctype: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_enctype_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_enctype =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_enctype);
                __widl_f_set_form_enctype_HTMLInputElement(self_, form_enctype)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_method_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formMethod` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn form_method(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_method_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_method_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_method_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_method_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formMethod` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formMethod)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_method(&self, form_method: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_method_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_method: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_method_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_method =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_method);
                __widl_f_set_form_method_HTMLInputElement(self_, form_method)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_no_validate_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formNoValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn form_no_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_no_validate_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_no_validate_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_no_validate_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_no_validate_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formNoValidate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formNoValidate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_no_validate(&self, form_no_validate: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_no_validate_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_no_validate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_no_validate_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_no_validate =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_no_validate);
                __widl_f_set_form_no_validate_HTMLInputElement(self_, form_no_validate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_target_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formTarget` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn form_target(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_target_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_target_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_target_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_form_target_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `formTarget` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/formTarget)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_form_target(&self, form_target: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_form_target_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                form_target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_form_target_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let form_target =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(form_target);
                __widl_f_set_form_target_HTMLInputElement(self_, form_target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLInputElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/height)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: u32) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLInputElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_indeterminate_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `indeterminate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn indeterminate(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_indeterminate_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_indeterminate_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_indeterminate_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_indeterminate_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `indeterminate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/indeterminate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_indeterminate(&self, indeterminate: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_indeterminate_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                indeterminate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_indeterminate_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            indeterminate: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(indeterminate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let indeterminate =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(indeterminate);
                __widl_f_set_indeterminate_HTMLInputElement(self_, indeterminate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_input_mode_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `inputMode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/inputMode)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn input_mode(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_input_mode_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_input_mode_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_input_mode_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_input_mode_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `inputMode` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/inputMode)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_input_mode(&self, input_mode: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_input_mode_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                input_mode: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_input_mode_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            input_mode: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(input_mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let input_mode = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(input_mode);
                __widl_f_set_input_mode_HTMLInputElement(self_, input_mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement", feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_list_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<HtmlElement> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlElement", feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `list` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/list)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn list(&self) -> Option<HtmlElement> {
        #[cfg(all(feature = "HtmlElement", feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_list_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_list_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_list_HTMLInputElement(self_)
            };
            <Option<HtmlElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `max` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn max(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `max` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/max)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_max(&self, max: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max);
                __widl_f_set_max_HTMLInputElement(self_, max)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_length_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `maxLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn max_length(&self) -> i32 {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_length_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_length_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_length_HTMLInputElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_length_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `maxLength` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/maxLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_max_length(&self, max_length: i32) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_length_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_length: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_length_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_length = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_length);
                __widl_f_set_max_length_HTMLInputElement(self_, max_length)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `min` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn min(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_min_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `min` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/min)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_min(&self, min: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_min_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                min: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_min_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            min: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(min);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let min = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(min);
                __widl_f_set_min_HTMLInputElement(self_, min)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_length_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `minLength` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn min_length(&self) -> i32 {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_length_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_length_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_length_HTMLInputElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_min_length_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `minLength` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/minLength)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_min_length(&self, min_length: i32) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_min_length_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                min_length: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_min_length_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let min_length = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(min_length);
                __widl_f_set_min_length_HTMLInputElement(self_, min_length)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multiple_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `multiple` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn multiple(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multiple_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multiple_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_multiple_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_multiple_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `multiple` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_multiple(&self, multiple: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_multiple_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                multiple: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_multiple_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            multiple: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(multiple);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let multiple = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(multiple);
                __widl_f_set_multiple_HTMLInputElement(self_, multiple)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/name)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLInputElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pattern_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `pattern` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn pattern(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pattern_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pattern_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pattern_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pattern_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `pattern` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/pattern)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_pattern(&self, pattern: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pattern_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pattern: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pattern_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pattern: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pattern);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pattern = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pattern);
                __widl_f_set_pattern_HTMLInputElement(self_, pattern)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_placeholder_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `placeholder` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn placeholder(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_placeholder_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_placeholder_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_placeholder_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_placeholder_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `placeholder` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/placeholder)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_placeholder(&self, placeholder: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_placeholder_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                placeholder: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_placeholder_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let placeholder =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(placeholder);
                __widl_f_set_placeholder_HTMLInputElement(self_, placeholder)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_only_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `readOnly` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn read_only(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_only_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_only_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_read_only_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_read_only_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `readOnly` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/readOnly)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_read_only(&self, read_only: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_read_only_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                read_only: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_read_only_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let read_only = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(read_only);
                __widl_f_set_read_only_HTMLInputElement(self_, read_only)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `required` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn required(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_required_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `required` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/required)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_required(&self, required: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_required_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                required: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_required_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let required = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(required);
                __widl_f_set_required_HTMLInputElement(self_, required)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> u32 {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_HTMLInputElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_size_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/size)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_size(&self, size: u32) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_size_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_size_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_set_size_HTMLInputElement(self_, size)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_src_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn src(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_src_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_src_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_src_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_src_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `src` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/src)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_src(&self, src: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_src_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_src_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_set_src_HTMLInputElement(self_, src)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_step_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `step` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn step(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_step_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_step_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_step_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_step_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `step` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/step)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_step(&self, step: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_step_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                step: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_step_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            step: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(step);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let step = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(step);
                __widl_f_set_step_HTMLInputElement(self_, step)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/type)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLInputElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_value_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn default_value(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_value_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_value_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_value_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_value_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultValue` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/defaultValue)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_value(&self, default_value: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_value_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_value_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_value =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_value);
                __widl_f_set_default_value_HTMLInputElement(self_, default_value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/value)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLInputElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_as_number_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `valueAsNumber` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn value_as_number(&self) -> f64 {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_as_number_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_as_number_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_as_number_HTMLInputElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_as_number_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `valueAsNumber` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/valueAsNumber)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_value_as_number(&self, value_as_number: f64) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_as_number_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value_as_number: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_as_number_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value_as_number: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(value_as_number);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value_as_number =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value_as_number);
                __widl_f_set_value_as_number_HTMLInputElement(self_, value_as_number)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLInputElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/width)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: u32) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(width);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLInputElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlInputElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLInputElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLInputElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlInputElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<NodeList> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> Option<NodeList> {
        #[cfg(all(feature = "HtmlInputElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<NodeList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<NodeList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLInputElement(self_)
            };
            <Option<NodeList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_start_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn selection_start(&self) -> Result<Option<u32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_start_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_start_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_start_HTMLInputElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<u32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_start_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionStart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionStart)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_start(
        &self,
        selection_start: Option<u32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_start_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selection_start: <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_start_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selection_start =
                    <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selection_start);
                __widl_f_set_selection_start_HTMLInputElement(self_, selection_start)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_end_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn selection_end(&self) -> Result<Option<u32>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_end_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u32> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_end_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_end_HTMLInputElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<u32> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_end_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<u32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionEnd` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionEnd)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_end(
        &self,
        selection_end: Option<u32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_end_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selection_end: <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_end_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selection_end =
                    <Option<u32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selection_end);
                __widl_f_set_selection_end_HTMLInputElement(self_, selection_end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selection_direction_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<String> as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionDirection` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn selection_direction(&self) -> Result<Option<String>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selection_direction_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<String> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selection_direction_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selection_direction_HTMLInputElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<String> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selection_direction_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectionDirection` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/selectionDirection)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_selection_direction(
        &self,
        selection_direction: Option<&str>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selection_direction_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selection_direction: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selection_direction_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selection_direction =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        selection_direction,
                    );
                __widl_f_set_selection_direction_HTMLInputElement(self_, selection_direction)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/align)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(align);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLInputElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_use_map_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `useMap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn use_map(&self) -> String {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_use_map_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_use_map_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_use_map_HTMLInputElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_use_map_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `useMap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_use_map(&self, use_map: &str) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_use_map_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                use_map: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_use_map_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            use_map: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(use_map);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let use_map = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(use_map);
                __widl_f_set_use_map_HTMLInputElement(self_, use_map)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_webkit_entries_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `webkitEntries` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitEntries)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn webkit_entries(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_webkit_entries_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_webkit_entries_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_webkit_entries_HTMLInputElement(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_webkitdirectory_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `webkitdirectory` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn webkitdirectory(&self) -> bool {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_webkitdirectory_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_webkitdirectory_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_webkitdirectory_HTMLInputElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlInputElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_webkitdirectory_HTMLInputElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlInputElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlInputElement {
    #[cfg(all(feature = "HtmlInputElement",))]
    #[allow(bad_style)]
    #[doc = "The `webkitdirectory` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory)\n\n*This API requires the following crate features to be activated: `HtmlInputElement`*"]
    #[allow(clippy::all)]
    pub fn set_webkitdirectory(&self, webkitdirectory: bool) {
        #[cfg(all(feature = "HtmlInputElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_webkitdirectory_HTMLInputElement(
                self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                webkitdirectory: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_webkitdirectory_HTMLInputElement(
            self_: <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            webkitdirectory: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(webkitdirectory);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlInputElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let webkitdirectory =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(webkitdirectory);
                __widl_f_set_webkitdirectory_HTMLInputElement(self_, webkitdirectory)
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
pub static __WASM_BINDGEN_GENERATED_64bb163f91d2b95e: [u8; 9374usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\\$\0\0\0\0`\0\0\x02\x10HTMLInputElement\"__widl_instanceof_HTMLInputElement\0\0\0\0(__widl_f_check_validity_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0)__widl_f_report_validity_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0 __widl_f_select_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x01\x05self_\x06select\0\0\0-__widl_f_set_custom_validity_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0(__widl_f_set_range_text_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x02\x05self_\x0Breplacement\x0CsetRangeText\0\0\0;__widl_f_set_range_text_with_start_and_end_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x04\x05self_\x0Breplacement\x05start\x03end\x0CsetRangeText\0\0\0-__widl_f_set_selection_range_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x03\x05self_\x05start\x03end\x11setSelectionRange\0\0\0<__widl_f_set_selection_range_with_direction_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\0\x01\x04\x05self_\x05start\x03end\tdirection\x11setSelectionRange\0\0\0 __widl_f_accept_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x06accept\x01\x01\x05self_\x06accept\0\0\0$__widl_f_set_accept_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x06accept\x01\x02\x05self_\x06accept\x06accept\0\0\0\x1D__widl_f_alt_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x03alt\x01\x01\x05self_\x03alt\0\0\0!__widl_f_set_alt_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x03alt\x01\x02\x05self_\x03alt\x03alt\0\0\0&__widl_f_autocomplete_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0Cautocomplete\x01\x01\x05self_\x0Cautocomplete\0\0\0*__widl_f_set_autocomplete_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0Cautocomplete\x01\x02\x05self_\x0Cautocomplete\x0Cautocomplete\0\0\0#__widl_f_autofocus_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\tautofocus\x01\x01\x05self_\tautofocus\0\0\0'__widl_f_set_autofocus_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\tautofocus\x01\x02\x05self_\tautofocus\tautofocus\0\0\0)__widl_f_default_checked_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0EdefaultChecked\x01\x01\x05self_\x0EdefaultChecked\0\0\0-__widl_f_set_default_checked_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0EdefaultChecked\x01\x02\x05self_\x0Fdefault_checked\x0EdefaultChecked\0\0\0!__widl_f_checked_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x07checked\x01\x01\x05self_\x07checked\0\0\0%__widl_f_set_checked_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x07checked\x01\x02\x05self_\x07checked\x07checked\0\0\0\"__widl_f_disabled_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0&__widl_f_set_disabled_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0\x1E__widl_f_form_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0\x1F__widl_f_files_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x05files\x01\x01\x05self_\x05files\0\0\0#__widl_f_set_files_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x05files\x01\x02\x05self_\x05files\x05files\0\0\0%__widl_f_form_action_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\nformAction\x01\x01\x05self_\nformAction\0\0\0)__widl_f_set_form_action_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\nformAction\x01\x02\x05self_\x0Bform_action\nformAction\0\0\0&__widl_f_form_enctype_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0BformEnctype\x01\x01\x05self_\x0BformEnctype\0\0\0*__widl_f_set_form_enctype_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0BformEnctype\x01\x02\x05self_\x0Cform_enctype\x0BformEnctype\0\0\0%__widl_f_form_method_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\nformMethod\x01\x01\x05self_\nformMethod\0\0\0)__widl_f_set_form_method_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\nformMethod\x01\x02\x05self_\x0Bform_method\nformMethod\0\0\0*__widl_f_form_no_validate_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0EformNoValidate\x01\x01\x05self_\x0EformNoValidate\0\0\0.__widl_f_set_form_no_validate_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0EformNoValidate\x01\x02\x05self_\x10form_no_validate\x0EformNoValidate\0\0\0%__widl_f_form_target_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\nformTarget\x01\x01\x05self_\nformTarget\0\0\0)__widl_f_set_form_target_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\nformTarget\x01\x02\x05self_\x0Bform_target\nformTarget\0\0\0 __widl_f_height_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0$__widl_f_set_height_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0'__widl_f_indeterminate_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\rindeterminate\x01\x01\x05self_\rindeterminate\0\0\0+__widl_f_set_indeterminate_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\rindeterminate\x01\x02\x05self_\rindeterminate\rindeterminate\0\0\0$__widl_f_input_mode_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\tinputMode\x01\x01\x05self_\tinputMode\0\0\0(__widl_f_set_input_mode_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\tinputMode\x01\x02\x05self_\ninput_mode\tinputMode\0\0\0\x1E__widl_f_list_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x04list\x01\x01\x05self_\x04list\0\0\0\x1D__widl_f_max_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x03max\x01\x01\x05self_\x03max\0\0\0!__widl_f_set_max_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x03max\x01\x02\x05self_\x03max\x03max\0\0\0$__widl_f_max_length_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\tmaxLength\x01\x01\x05self_\tmaxLength\0\0\0(__widl_f_set_max_length_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\tmaxLength\x01\x02\x05self_\nmax_length\tmaxLength\0\0\0\x1D__widl_f_min_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x03min\x01\x01\x05self_\x03min\0\0\0!__widl_f_set_min_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x03min\x01\x02\x05self_\x03min\x03min\0\0\0$__widl_f_min_length_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\tminLength\x01\x01\x05self_\tminLength\0\0\0(__widl_f_set_min_length_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\tminLength\x01\x02\x05self_\nmin_length\tminLength\0\0\0\"__widl_f_multiple_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x08multiple\x01\x01\x05self_\x08multiple\0\0\0&__widl_f_set_multiple_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x08multiple\x01\x02\x05self_\x08multiple\x08multiple\0\0\0\x1E__widl_f_name_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\"__widl_f_set_name_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0!__widl_f_pattern_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x07pattern\x01\x01\x05self_\x07pattern\0\0\0%__widl_f_set_pattern_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x07pattern\x01\x02\x05self_\x07pattern\x07pattern\0\0\0%__widl_f_placeholder_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0Bplaceholder\x01\x01\x05self_\x0Bplaceholder\0\0\0)__widl_f_set_placeholder_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0Bplaceholder\x01\x02\x05self_\x0Bplaceholder\x0Bplaceholder\0\0\0#__widl_f_read_only_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x08readOnly\x01\x01\x05self_\x08readOnly\0\0\0'__widl_f_set_read_only_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x08readOnly\x01\x02\x05self_\tread_only\x08readOnly\0\0\0\"__widl_f_required_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x08required\x01\x01\x05self_\x08required\0\0\0&__widl_f_set_required_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x08required\x01\x02\x05self_\x08required\x08required\0\0\0\x1E__widl_f_size_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\"__widl_f_set_size_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x04size\x01\x02\x05self_\x04size\x04size\0\0\0\x1D__widl_f_src_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x03src\x01\x01\x05self_\x03src\0\0\0!__widl_f_set_src_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x03src\x01\x02\x05self_\x03src\x03src\0\0\0\x1E__widl_f_step_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x04step\x01\x01\x05self_\x04step\0\0\0\"__widl_f_set_step_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x04step\x01\x02\x05self_\x04step\x04step\0\0\0\x1E__widl_f_type_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\"__widl_f_set_type_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0'__widl_f_default_value_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0CdefaultValue\x01\x01\x05self_\x0CdefaultValue\0\0\0+__widl_f_set_default_value_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0CdefaultValue\x01\x02\x05self_\rdefault_value\x0CdefaultValue\0\0\0\x1F__widl_f_value_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0#__widl_f_set_value_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0)__widl_f_value_as_number_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\rvalueAsNumber\x01\x01\x05self_\rvalueAsNumber\0\0\0-__widl_f_set_value_as_number_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\rvalueAsNumber\x01\x02\x05self_\x0Fvalue_as_number\rvalueAsNumber\0\0\0\x1F__widl_f_width_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0#__widl_f_set_width_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0'__widl_f_will_validate_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0\"__widl_f_validity_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0,__widl_f_validation_message_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0 __widl_f_labels_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0)__widl_f_selection_start_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x01\x0EselectionStart\x01\x01\x05self_\x0EselectionStart\0\0\0-__widl_f_set_selection_start_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x02\x0EselectionStart\x01\x02\x05self_\x0Fselection_start\x0EselectionStart\0\0\0'__widl_f_selection_end_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x01\x0CselectionEnd\x01\x01\x05self_\x0CselectionEnd\0\0\0+__widl_f_set_selection_end_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x02\x0CselectionEnd\x01\x02\x05self_\rselection_end\x0CselectionEnd\0\0\0-__widl_f_selection_direction_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x01\x12selectionDirection\x01\x01\x05self_\x12selectionDirection\0\0\01__widl_f_set_selection_direction_HTMLInputElement\x01\0\0\x01\x10HTMLInputElement\x01\0\x02\x12selectionDirection\x01\x02\x05self_\x13selection_direction\x12selectionDirection\0\0\0\x1F__widl_f_align_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0#__widl_f_set_align_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0!__widl_f_use_map_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x06useMap\x01\x01\x05self_\x06useMap\0\0\0%__widl_f_set_use_map_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x06useMap\x01\x02\x05self_\x07use_map\x06useMap\0\0\0(__widl_f_webkit_entries_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\rwebkitEntries\x01\x01\x05self_\rwebkitEntries\0\0\0)__widl_f_webkitdirectory_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x01\x0Fwebkitdirectory\x01\x01\x05self_\x0Fwebkitdirectory\0\0\0-__widl_f_set_webkitdirectory_HTMLInputElement\0\0\0\x01\x10HTMLInputElement\x01\0\x02\x0Fwebkitdirectory\x01\x02\x05self_\x0Fwebkitdirectory\x0Fwebkitdirectory\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
