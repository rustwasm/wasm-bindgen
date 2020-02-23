use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLObjectElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlObjectElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlObjectElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlObjectElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(79u32);
            inform(98u32);
            inform(106u32);
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
    impl core::ops::Deref for HtmlObjectElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlObjectElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlObjectElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlObjectElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlObjectElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlObjectElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlObjectElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlObjectElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlObjectElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlObjectElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlObjectElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlObjectElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlObjectElement {
            HtmlObjectElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlObjectElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlObjectElement> for HtmlObjectElement {
        #[inline]
        fn as_ref(&self) -> &HtmlObjectElement {
            self
        }
    }
    impl From<HtmlObjectElement> for JsValue {
        #[inline]
        fn from(obj: HtmlObjectElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlObjectElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLObjectElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLObjectElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLObjectElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlObjectElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlObjectElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlObjectElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlObjectElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlObjectElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlObjectElement> for Element {
    #[inline]
    fn from(obj: HtmlObjectElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlObjectElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlObjectElement> for Node {
    #[inline]
    fn from(obj: HtmlObjectElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlObjectElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlObjectElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlObjectElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlObjectElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlObjectElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlObjectElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlObjectElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_check_validity_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `checkValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/checkValidity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn check_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_check_validity_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_check_validity_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_check_validity_HTMLObjectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_svg_document_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "Document", feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `getSVGDocument()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/getSVGDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn get_svg_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_svg_document_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_svg_document_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_svg_document_HTMLObjectElement(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_report_validity_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `reportValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/reportValidity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn report_validity(&self) -> bool {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_report_validity_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_report_validity_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_report_validity_HTMLObjectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_custom_validity_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `setCustomValidity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/setCustomValidity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_custom_validity(&self, error: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_custom_validity_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_custom_validity_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(error);
                __widl_f_set_custom_validity_HTMLObjectElement(self_, error)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_data_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `data` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn data(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_data_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_data_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_data_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_data_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `data` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_data(&self, data: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_data_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_data_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_set_data_HTMLObjectElement(self_, data)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLObjectElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_must_match_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `typeMustMatch` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn type_must_match(&self) -> bool {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_must_match_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_must_match_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_must_match_HTMLObjectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_must_match_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `typeMustMatch` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_type_must_match(&self, type_must_match: bool) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_must_match_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_must_match: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_must_match_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_must_match: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_must_match);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_must_match =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_must_match);
                __widl_f_set_type_must_match_HTMLObjectElement(self_, type_must_match)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLObjectElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_use_map_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `useMap` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn use_map(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_use_map_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_use_map_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_use_map_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_use_map_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `useMap` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_use_map(&self, use_map: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_use_map_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                use_map: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_use_map_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let use_map = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(use_map);
                __widl_f_set_use_map_HTMLObjectElement(self_, use_map)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFormElement", feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_form_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <Option<HtmlFormElement> as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlFormElement", feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `form` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/form)\n\n*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn form(&self) -> Option<HtmlFormElement> {
        #[cfg(all(feature = "HtmlFormElement", feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_form_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_form_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_form_HTMLObjectElement(self_)
            };
            <Option<HtmlFormElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLObjectElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLObjectElement(self_, height)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Document", feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_document_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <Option<Document> as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "Document", feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `contentDocument` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentDocument)\n\n*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn content_document(&self) -> Option<Document> {
        #[cfg(all(feature = "Document", feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_document_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_document_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_document_HTMLObjectElement(self_)
            };
            <Option<Document> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement", feature = "Window",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_window_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <Option<Window> as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement", feature = "Window",))]
    #[allow(bad_style)]
    #[doc = "The `contentWindow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentWindow)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`, `Window`*"]
    #[allow(clippy::all)]
    pub fn content_window(&self) -> Option<Window> {
        #[cfg(all(feature = "HtmlObjectElement", feature = "Window",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_window_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_window_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_window_HTMLObjectElement(self_)
            };
            <Option<Window> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_will_validate_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `willValidate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/willValidate)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn will_validate(&self) -> bool {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_will_validate_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_will_validate_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_will_validate_HTMLObjectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement", feature = "ValidityState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validity_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <ValidityState as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement", feature = "ValidityState",))]
    #[allow(bad_style)]
    #[doc = "The `validity` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validity)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`, `ValidityState`*"]
    #[allow(clippy::all)]
    pub fn validity(&self) -> ValidityState {
        #[cfg(all(feature = "HtmlObjectElement", feature = "ValidityState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validity_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ValidityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validity_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validity_HTMLObjectElement(self_)
            };
            <ValidityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_validation_message_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `validationMessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validationMessage)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn validation_message(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_validation_message_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_validation_message_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_validation_message_HTMLObjectElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLObjectElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_archive_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `archive` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn archive(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_archive_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_archive_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_archive_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_archive_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `archive` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_archive(&self, archive: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_archive_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                archive: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_archive_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            archive: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(archive);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let archive = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(archive);
                __widl_f_set_archive_HTMLObjectElement(self_, archive)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `code` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn code(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_code_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `code` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_code(&self, code: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_code_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                code: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_code_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            code: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(code);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let code = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(code);
                __widl_f_set_code_HTMLObjectElement(self_, code)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_declare_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `declare` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn declare(&self) -> bool {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_declare_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_declare_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_declare_HTMLObjectElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_declare_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `declare` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_declare(&self, declare: bool) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_declare_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                declare: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_declare_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            declare: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(declare);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let declare = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(declare);
                __widl_f_set_declare_HTMLObjectElement(self_, declare)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hspace_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `hspace` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn hspace(&self) -> u32 {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hspace_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hspace_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hspace_HTMLObjectElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hspace_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `hspace` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_hspace(&self, hspace: u32) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hspace_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hspace_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hspace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hspace = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hspace);
                __widl_f_set_hspace_HTMLObjectElement(self_, hspace)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_standby_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `standby` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn standby(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_standby_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_standby_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_standby_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_standby_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `standby` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_standby(&self, standby: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_standby_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                standby: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_standby_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            standby: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(standby);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let standby = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(standby);
                __widl_f_set_standby_HTMLObjectElement(self_, standby)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_vspace_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `vspace` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn vspace(&self) -> u32 {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_vspace_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_vspace_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_vspace_HTMLObjectElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_vspace_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `vspace` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_vspace(&self, vspace: u32) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_vspace_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                vspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_vspace_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            vspace: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(vspace);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let vspace = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(vspace);
                __widl_f_set_vspace_HTMLObjectElement(self_, vspace)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_base_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `codeBase` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn code_base(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_base_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_base_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_base_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_code_base_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `codeBase` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_code_base(&self, code_base: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_code_base_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                code_base: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_code_base_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            code_base: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(code_base);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let code_base = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(code_base);
                __widl_f_set_code_base_HTMLObjectElement(self_, code_base)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_code_type_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `codeType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn code_type(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_code_type_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_code_type_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_code_type_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_code_type_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `codeType` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_code_type(&self, code_type: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_code_type_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                code_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_code_type_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            code_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(code_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let code_type = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(code_type);
                __widl_f_set_code_type_HTMLObjectElement(self_, code_type)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_border_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `border` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn border(&self) -> String {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_border_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_border_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_border_HTMLObjectElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlObjectElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_border_HTMLObjectElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlObjectElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlObjectElement {
    #[cfg(all(feature = "HtmlObjectElement",))]
    #[allow(bad_style)]
    #[doc = "The `border` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)\n\n*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
    #[allow(clippy::all)]
    pub fn set_border(&self, border: &str) {
        #[cfg(all(feature = "HtmlObjectElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_border_HTMLObjectElement(
                self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_border_HTMLObjectElement(
            self_: <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            border: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(border);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlObjectElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let border = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(border);
                __widl_f_set_border_HTMLObjectElement(self_, border)
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
pub static __WASM_BINDGEN_GENERATED_08a17a584404dcf9: [u8; 4245usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}S\x10\0\0\0\0-\0\0\x02\x11HTMLObjectElement#__widl_instanceof_HTMLObjectElement\0\0\0\0)__widl_f_check_validity_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\0\x01\x01\x05self_\rcheckValidity\0\0\0+__widl_f_get_svg_document_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\0\x01\x01\x05self_\x0EgetSVGDocument\0\0\0*__widl_f_report_validity_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\0\x01\x01\x05self_\x0EreportValidity\0\0\0.__widl_f_set_custom_validity_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\0\x01\x02\x05self_\x05error\x11setCustomValidity\0\0\0\x1F__widl_f_data_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x04data\x01\x01\x05self_\x04data\0\0\0#__widl_f_set_data_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x04data\x01\x02\x05self_\x04data\x04data\0\0\0\x1F__widl_f_type_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0#__widl_f_set_type_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0*__widl_f_type_must_match_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\rtypeMustMatch\x01\x01\x05self_\rtypeMustMatch\0\0\0.__widl_f_set_type_must_match_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\rtypeMustMatch\x01\x02\x05self_\x0Ftype_must_match\rtypeMustMatch\0\0\0\x1F__widl_f_name_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_set_name_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\"__widl_f_use_map_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x06useMap\x01\x01\x05self_\x06useMap\0\0\0&__widl_f_set_use_map_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x06useMap\x01\x02\x05self_\x07use_map\x06useMap\0\0\0\x1F__widl_f_form_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x04form\x01\x01\x05self_\x04form\0\0\0 __widl_f_width_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0$__widl_f_set_width_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0!__widl_f_height_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0%__widl_f_set_height_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0+__widl_f_content_document_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x0FcontentDocument\x01\x01\x05self_\x0FcontentDocument\0\0\0)__widl_f_content_window_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\rcontentWindow\x01\x01\x05self_\rcontentWindow\0\0\0(__widl_f_will_validate_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x0CwillValidate\x01\x01\x05self_\x0CwillValidate\0\0\0#__widl_f_validity_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x08validity\x01\x01\x05self_\x08validity\0\0\0-__widl_f_validation_message_HTMLObjectElement\x01\0\0\x01\x11HTMLObjectElement\x01\0\x01\x11validationMessage\x01\x01\x05self_\x11validationMessage\0\0\0 __widl_f_align_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0$__widl_f_set_align_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\"__widl_f_archive_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x07archive\x01\x01\x05self_\x07archive\0\0\0&__widl_f_set_archive_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x07archive\x01\x02\x05self_\x07archive\x07archive\0\0\0\x1F__widl_f_code_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x04code\x01\x01\x05self_\x04code\0\0\0#__widl_f_set_code_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x04code\x01\x02\x05self_\x04code\x04code\0\0\0\"__widl_f_declare_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x07declare\x01\x01\x05self_\x07declare\0\0\0&__widl_f_set_declare_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x07declare\x01\x02\x05self_\x07declare\x07declare\0\0\0!__widl_f_hspace_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x06hspace\x01\x01\x05self_\x06hspace\0\0\0%__widl_f_set_hspace_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x06hspace\x01\x02\x05self_\x06hspace\x06hspace\0\0\0\"__widl_f_standby_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x07standby\x01\x01\x05self_\x07standby\0\0\0&__widl_f_set_standby_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x07standby\x01\x02\x05self_\x07standby\x07standby\0\0\0!__widl_f_vspace_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x06vspace\x01\x01\x05self_\x06vspace\0\0\0%__widl_f_set_vspace_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x06vspace\x01\x02\x05self_\x06vspace\x06vspace\0\0\0$__widl_f_code_base_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x08codeBase\x01\x01\x05self_\x08codeBase\0\0\0(__widl_f_set_code_base_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x08codeBase\x01\x02\x05self_\tcode_base\x08codeBase\0\0\0$__widl_f_code_type_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x08codeType\x01\x01\x05self_\x08codeType\0\0\0(__widl_f_set_code_type_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x08codeType\x01\x02\x05self_\tcode_type\x08codeType\0\0\0!__widl_f_border_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x01\x06border\x01\x01\x05self_\x06border\0\0\0%__widl_f_set_border_HTMLObjectElement\0\0\0\x01\x11HTMLObjectElement\x01\0\x02\x06border\x01\x02\x05self_\x06border\x06border\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
