use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLSelectElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlSelectElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlSelectElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlSelectElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(83u32);
            inform(101u32);
            inform(108u32);
            inform(101u32);
            inform(99u32);
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
    impl core::ops::Deref for HtmlSelectElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlSelectElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlSelectElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlSelectElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlSelectElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlSelectElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlSelectElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlSelectElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlSelectElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlSelectElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlSelectElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlSelectElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlSelectElement {
            HtmlSelectElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlSelectElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlSelectElement> for HtmlSelectElement {
        #[inline]
        fn as_ref(&self) -> &HtmlSelectElement {
            self
        }
    }
    impl From<HtmlSelectElement> for JsValue {
        #[inline]
        fn from(obj: HtmlSelectElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlSelectElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLSelectElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLSelectElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLSelectElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlSelectElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlSelectElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlSelectElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlSelectElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlSelectElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSelectElement> for Element {
    #[inline]
    fn from(obj: HtmlSelectElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlSelectElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSelectElement> for Node {
    #[inline]
    fn from(obj: HtmlSelectElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlSelectElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSelectElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlSelectElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlSelectElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlSelectElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlSelectElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlSelectElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_option_element_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&HtmlOptionElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_option_element(
        &self,
        element: &HtmlOptionElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_option_element_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_option_element_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_add_with_html_option_element_HTMLSelectElement(self_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_opt_group_element_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&HtmlOptGroupElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_opt_group_element(
        &self,
        element: &HtmlOptGroupElement,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_opt_group_element_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_opt_group_element_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                __widl_f_add_with_html_opt_group_element_HTMLSelectElement(self_, element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "HtmlElement",
    feature = "HtmlOptionElement",
    feature = "HtmlSelectElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_option_element_and_opt_html_element_HTMLSelectElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&HtmlOptionElement as WasmDescribe>::describe();
    <Option<&HtmlElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(
        feature = "HtmlElement",
        feature = "HtmlOptionElement",
        feature = "HtmlSelectElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptionElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_option_element_and_opt_html_element(
        &self,
        element: &HtmlOptionElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "HtmlElement",
            feature = "HtmlOptionElement",
            feature = "HtmlSelectElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_option_element_and_opt_html_element_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_option_element_and_opt_html_element_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before =
                    <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_option_element_and_opt_html_element_HTMLSelectElement(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(
    feature = "HtmlElement",
    feature = "HtmlOptGroupElement",
    feature = "HtmlSelectElement",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLSelectElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&HtmlOptGroupElement as WasmDescribe>::describe();
    <Option<&HtmlElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(
        feature = "HtmlElement",
        feature = "HtmlOptGroupElement",
        feature = "HtmlSelectElement",
    ))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlElement`, `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_opt_group_element_and_opt_html_element(
        &self,
        element: &HtmlOptGroupElement,
        before: Option<&HtmlElement>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "HtmlElement",
            feature = "HtmlOptGroupElement",
            feature = "HtmlSelectElement",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before =
                    <Option<&HtmlElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLSelectElement(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_option_element_and_opt_i32_HTMLSelectElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&HtmlOptionElement as WasmDescribe>::describe();
    <Option<i32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_option_element_and_opt_i32(
        &self,
        element: &HtmlOptionElement,
        before: Option<i32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_option_element_and_opt_i32_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_option_element_and_opt_i32_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptionElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before = <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_option_element_and_opt_i32_HTMLSelectElement(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLSelectElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&HtmlOptGroupElement as WasmDescribe>::describe();
    <Option<i32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `add()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)\n\n*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn add_with_html_opt_group_element_and_opt_i32(
        &self,
        element: &HtmlOptGroupElement,
        before: Option<i32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptGroupElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            element: <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            before: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(element);
            drop(before);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let element =
                    <&HtmlOptGroupElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(element);
                let before = <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(before);
                __widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLSelectElement(
                    self_, element, before,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_item_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "Element", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `item()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn item(&self, index: u32) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_item_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_item_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_item_HTMLSelectElement(self_, index)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_named_item_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<HtmlOptionElement> as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `namedItem()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem)\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn named_item(&self, name: &str) -> Option<HtmlOptionElement> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_named_item_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlOptionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_named_item_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<HtmlOptionElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_named_item_HTMLSelectElement(self_, name)
            };
            <Option<HtmlOptionElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_with_index_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn remove_with_index(&self, index: i32) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_with_index_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_with_index_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_remove_with_index_HTMLSelectElement(self_, index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn remove(&self) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_HTMLSelectElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLSelectElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "Element", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The indexing getter\n\n\n\n*This API requires the following crate features to be activated: `Element`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn get(&self, index: u32) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                __widl_f_get_HTMLSelectElement(self_, index)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&HtmlOptionElement> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The indexing setter\n\n\n\n*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set(
        &self,
        index: u32,
        option: Option<&HtmlOptionElement>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlOptionElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                option: <Option<&HtmlOptionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            index: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            option: <Option<&HtmlOptionElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(index);
            drop(option);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let index = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(index);
                let option =
                    <Option<&HtmlOptionElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        option,
                    );
                __widl_f_set_HTMLSelectElement(self_, index, option)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autofocus_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn autofocus(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autofocus_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autofocus_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autofocus_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autofocus_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `autofocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_autofocus(&self, autofocus: bool) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autofocus_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autofocus: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autofocus_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autofocus = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autofocus);
                __widl_f_set_autofocus_HTMLSelectElement(self_, autofocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_autocomplete_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn autocomplete(&self) -> String {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_autocomplete_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_autocomplete_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_autocomplete_HTMLSelectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_autocomplete_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `autocomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_autocomplete(&self, autocomplete: &str) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_autocomplete_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                autocomplete: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_autocomplete_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let autocomplete =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(autocomplete);
                __widl_f_set_autocomplete_HTMLSelectElement(self_, autocomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLSelectElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLSelectElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_multiple_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `multiple` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn multiple(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_multiple_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_multiple_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_multiple_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_multiple_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `multiple` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_multiple(&self, multiple: bool) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_multiple_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                multiple: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_multiple_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let multiple = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(multiple);
                __widl_f_set_multiple_HTMLSelectElement(self_, multiple)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLSelectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLSelectElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `required` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn required(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_required_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `required` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_required(&self, required: bool) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_required_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                required: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_required_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let required = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(required);
                __widl_f_set_required_HTMLSelectElement(self_, required)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> u32 {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_HTMLSelectElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_size_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_size(&self, size: u32) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_size_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_size_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_set_size_HTMLSelectElement(self_, size)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLSelectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlOptionsCollection", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_options_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <HtmlOptionsCollection as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlOptionsCollection", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `options` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options)\n\n*This API requires the following crate features to be activated: `HtmlOptionsCollection`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn options(&self) -> HtmlOptionsCollection {
        #[cfg(all(feature = "HtmlOptionsCollection", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_options_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlOptionsCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_options_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlOptionsCollection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_options_HTMLSelectElement(self_)
            };
            <HtmlOptionsCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_length_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `length` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn length(&self) -> u32 {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_length_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_length_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_length_HTMLSelectElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_length_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `length` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_length(&self, length: u32) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_length_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_length_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(length);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                __widl_f_set_length_HTMLSelectElement(self_, length)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlCollection", feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selected_options_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <HtmlCollection as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlCollection", feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectedOptions` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions)\n\n*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn selected_options(&self) -> HtmlCollection {
        #[cfg(all(feature = "HtmlCollection", feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selected_options_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selected_options_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selected_options_HTMLSelectElement(self_)
            };
            <HtmlCollection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_selected_index_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectedIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn selected_index(&self) -> i32 {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_selected_index_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_selected_index_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_selected_index_HTMLSelectElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_selected_index_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `selectedIndex` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_selected_index(&self, selected_index: i32) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_selected_index_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                selected_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_selected_index_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            selected_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(selected_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let selected_index =
                    <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(selected_index);
                __widl_f_set_selected_index_HTMLSelectElement(self_, selected_index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> String {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLSelectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: &str) {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLSelectElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLSelectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlSelectElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLSelectElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlSelectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLSelectElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlSelectElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLSelectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlSelectElement as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl HtmlSelectElement {
    #[cfg(all(feature = "HtmlSelectElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlSelectElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> NodeList {
        #[cfg(all(feature = "HtmlSelectElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLSelectElement(
                self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLSelectElement(
            self_: <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlSelectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLSelectElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_d4364310200469b4: [u8; 4345usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB7\x10\0\0\0\0,\0\0\x02\x11HTMLSelectElement#__widl_instanceof_HTMLSelectElement\0\0\0\07__widl_f_add_with_html_option_element_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x02\x05self_\x07element\x03add\0\0\0:__widl_f_add_with_html_opt_group_element_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x02\x05self_\x07element\x03add\0\0\0L__widl_f_add_with_html_option_element_and_opt_html_element_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0O__widl_f_add_with_html_opt_group_element_and_opt_html_element_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0C__widl_f_add_with_html_option_element_and_opt_i32_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0F__widl_f_add_with_html_opt_group_element_and_opt_i32_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x03\x05self_\x07element\x06before\x03add\0\0\0)__widl_f_check_validity_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0\x1F__widl_f_item_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x02\x05self_\x05index\x04item\0\0\0%__widl_f_named_item_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x02\x05self_\x04name\tnamedItem\0\0\0,__widl_f_remove_with_index_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x02\x05self_\x05index\x06remove\0\0\0!__widl_f_remove_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x01\x05self_\x06remove\0\0\0*__widl_f_report_validity_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0.__widl_f_set_custom_validity_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0\x1E__widl_f_get_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x03\x01\x02\x05self_\x05index\x03get\0\0\0\x1E__widl_f_set_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\x04\x01\x03\x05self_\x05index\x06option\x03set\0\0\0$__widl_f_autofocus_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\tautofocus\x01\x01\x05self_\tautofocus\0\0\0(__widl_f_set_autofocus_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\tautofocus\x01\x02\x05self_\tautofocus\tautofocus\0\0\0'__widl_f_autocomplete_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x0Cautocomplete\x01\x01\x05self_\x0Cautocomplete\0\0\0+__widl_f_set_autocomplete_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x0Cautocomplete\x01\x02\x05self_\x0Cautocomplete\x0Cautocomplete\0\0\0#__widl_f_disabled_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0'__widl_f_set_disabled_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0\x1F__widl_f_form_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0#__widl_f_multiple_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x08multiple\x01\x01\x05self_\x08multiple\0\0\0'__widl_f_set_multiple_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x08multiple\x01\x02\x05self_\x08multiple\x08multiple\0\0\0\x1F__widl_f_name_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_set_name_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0#__widl_f_required_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x08required\x01\x01\x05self_\x08required\0\0\0'__widl_f_set_required_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x08required\x01\x02\x05self_\x08required\x08required\0\0\0\x1F__widl_f_size_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0#__widl_f_set_size_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x04size\x01\x02\x05self_\x04size\x04size\0\0\0\x1F__widl_f_type_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\"__widl_f_options_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x07options\x01\x01\x05self_\x07options\0\0\0!__widl_f_length_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x06length\x01\x01\x05self_\x06length\0\0\0%__widl_f_set_length_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x06length\x01\x02\x05self_\x06length\x06length\0\0\0+__widl_f_selected_options_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x0FselectedOptions\x01\x01\x05self_\x0FselectedOptions\0\0\0)__widl_f_selected_index_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\rselectedIndex\x01\x01\x05self_\rselectedIndex\0\0\0-__widl_f_set_selected_index_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\rselectedIndex\x01\x02\x05self_\x0Eselected_index\rselectedIndex\0\0\0 __widl_f_value_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0$__widl_f_set_value_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0(__widl_f_will_validate_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0#__widl_f_validity_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0-__widl_f_validation_message_HTMLSelectElement\x01\0\0\x01\x11HTMLSelectElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0!__widl_f_labels_HTMLSelectElement\0\0\0\x01\x11HTMLSelectElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
