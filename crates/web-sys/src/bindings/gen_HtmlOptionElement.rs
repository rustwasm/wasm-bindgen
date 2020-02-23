use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLOptionElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlOptionElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlOptionElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlOptionElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(79u32);
            inform(112u32);
            inform(116u32);
            inform(105u32);
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
    impl core::ops::Deref for HtmlOptionElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlOptionElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlOptionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlOptionElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlOptionElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlOptionElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlOptionElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlOptionElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlOptionElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlOptionElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlOptionElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlOptionElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlOptionElement {
            HtmlOptionElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlOptionElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlOptionElement> for HtmlOptionElement {
        #[inline]
        fn as_ref(&self) -> &HtmlOptionElement {
            self
        }
    }
    impl From<HtmlOptionElement> for JsValue {
        #[inline]
        fn from(obj: HtmlOptionElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlOptionElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLOptionElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLOptionElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLOptionElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlOptionElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlOptionElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlOptionElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlOptionElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlOptionElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOptionElement> for Element {
    #[inline]
    fn from(obj: HtmlOptionElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlOptionElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOptionElement> for Node {
    #[inline]
    fn from(obj: HtmlOptionElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlOptionElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOptionElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlOptionElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlOptionElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlOptionElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlOptionElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlOptionElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Option() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <HtmlOptionElement as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLOptionElement(..)` constructor, creating a new instance of `HTMLOptionElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<HtmlOptionElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Option(
            ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Option(
        ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Option() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_text_Option() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <HtmlOptionElement as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLOptionElement(..)` constructor, creating a new instance of `HTMLOptionElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_text(text: &str) -> Result<HtmlOptionElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_text_Option(
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_text_Option(
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_new_with_text_Option(text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_text_and_value_Option() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <HtmlOptionElement as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLOptionElement(..)` constructor, creating a new instance of `HTMLOptionElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_text_and_value(
        text: &str,
        value: &str,
    ) -> Result<HtmlOptionElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_text_and_value_Option(
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_text_and_value_Option(
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(text);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_new_with_text_and_value_Option(text, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_text_and_value_and_default_selected_Option()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <HtmlOptionElement as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLOptionElement(..)` constructor, creating a new instance of `HTMLOptionElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_text_and_value_and_default_selected(
        text: &str,
        value: &str,
        default_selected: bool,
    ) -> Result<HtmlOptionElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_text_and_value_and_default_selected_Option(
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_text_and_value_and_default_selected_Option(
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(text);
            drop(value);
            drop(default_selected);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let default_selected =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_selected);
                __widl_f_new_with_text_and_value_and_default_selected_Option(
                    text,
                    value,
                    default_selected,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_text_and_value_and_default_selected_and_selected_Option(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <HtmlOptionElement as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLOptionElement(..)` constructor, creating a new instance of `HTMLOptionElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_text_and_value_and_default_selected_and_selected(
        text: &str,
        value: &str,
        default_selected: bool,
        selected: bool,
    ) -> Result<HtmlOptionElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_text_and_value_and_default_selected_and_selected_Option(
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_text_and_value_and_default_selected_and_selected_Option(
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(text);
            drop(value);
            drop(default_selected);
            drop(selected);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                let default_selected =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_selected);
                let selected = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selected);
                __widl_f_new_with_text_and_value_and_default_selected_and_selected_Option(
                    text,
                    value,
                    default_selected,
                    selected,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlOptionElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLOptionElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLOptionElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLOptionElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_HTMLOptionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_label_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `label` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn set_label(&self, label: &str) {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_label_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_label_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(label);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_set_label_HTMLOptionElement(self_, label)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_selected_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultSelected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn default_selected(&self) -> bool {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_selected_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_selected_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_selected_HTMLOptionElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_selected_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultSelected` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_selected(&self, default_selected: bool) {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_selected_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_selected_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default_selected);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_selected =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_selected);
                __widl_f_set_default_selected_HTMLOptionElement(self_, default_selected)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selected_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `selected` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn selected(&self) -> bool {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selected_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selected_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selected_HTMLOptionElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selected_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `selected` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn set_selected(&self, selected: bool) {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selected_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selected_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selected: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selected);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selected = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selected);
                __widl_f_set_selected_HTMLOptionElement(self_, selected)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLOptionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLOptionElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> String {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_HTMLOptionElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_HTMLOptionElement(self_, text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_index_HTMLOptionElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlOptionElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlOptionElement {
    #[cfg(all(feature = "HtmlOptionElement",))]
    #[allow(bad_style)]
    #[doc = "The `index` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/index)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
    #[allow(clippy::all)]
    pub fn index(&self) -> i32 {
        #[cfg(all(feature = "HtmlOptionElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_index_HTMLOptionElement(
                self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_index_HTMLOptionElement(
            self_: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_index_HTMLOptionElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_2583e37f4ae16538: [u8; 1873usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x0F\x07\0\0\0\0\x14\0\0\x02\x11HTMLOptionElement#__widl_instanceof_HTMLOptionElement\0\0\0\0\x13__widl_f_new_Option\x01\0\0\x01\x06Option\0\x01\0\x03new\0\0\0\x1D__widl_f_new_with_text_Option\x01\0\0\x01\x06Option\0\x01\x01\x04text\x03new\0\0\0'__widl_f_new_with_text_and_value_Option\x01\0\0\x01\x06Option\0\x01\x02\x04text\x05value\x03new\0\0\0<__widl_f_new_with_text_and_value_and_default_selected_Option\x01\0\0\x01\x06Option\0\x01\x03\x04text\x05value\x10default_selected\x03new\0\0\0I__widl_f_new_with_text_and_value_and_default_selected_and_selected_Option\x01\0\0\x01\x06Option\0\x01\x04\x04text\x05value\x10default_selected\x08selected\x03new\0\0\0#__widl_f_disabled_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0'__widl_f_set_disabled_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0\x1F__widl_f_form_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0 __widl_f_label_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0$__widl_f_set_label_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x02\x05label\x01\x02\x05self_\x05label\x05label\0\0\0+__widl_f_default_selected_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x0FdefaultSelected\x01\x01\x05self_\x0FdefaultSelected\0\0\0/__widl_f_set_default_selected_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x02\x0FdefaultSelected\x01\x02\x05self_\x10default_selected\x0FdefaultSelected\0\0\0#__widl_f_selected_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x08selected\x01\x01\x05self_\x08selected\0\0\0'__widl_f_set_selected_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x02\x08selected\x01\x02\x05self_\x08selected\x08selected\0\0\0 __widl_f_value_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0$__widl_f_set_value_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0\x1F__widl_f_text_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0#__widl_f_set_text_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0 __widl_f_index_HTMLOptionElement\0\0\0\x01\x11HTMLOptionElement\x01\0\x01\x05index\x01\x01\x05self_\x05index\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
