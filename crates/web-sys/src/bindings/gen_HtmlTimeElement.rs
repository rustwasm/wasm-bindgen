use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLTimeElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement)\n\n*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlTimeElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlTimeElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlTimeElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(84u32);
            inform(105u32);
            inform(109u32);
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
    impl core::ops::Deref for HtmlTimeElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlTimeElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlTimeElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlTimeElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlTimeElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlTimeElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlTimeElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlTimeElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlTimeElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlTimeElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlTimeElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlTimeElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlTimeElement {
            HtmlTimeElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlTimeElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlTimeElement> for HtmlTimeElement {
        #[inline]
        fn as_ref(&self) -> &HtmlTimeElement {
            self
        }
    }
    impl From<HtmlTimeElement> for JsValue {
        #[inline]
        fn from(obj: HtmlTimeElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlTimeElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLTimeElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLTimeElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLTimeElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlTimeElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlTimeElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlTimeElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlTimeElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlTimeElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTimeElement> for Element {
    #[inline]
    fn from(obj: HtmlTimeElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlTimeElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTimeElement> for Node {
    #[inline]
    fn from(obj: HtmlTimeElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlTimeElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTimeElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlTimeElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlTimeElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlTimeElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlTimeElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlTimeElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlTimeElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_date_time_HTMLTimeElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlTimeElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlTimeElement {
    #[cfg(all(feature = "HtmlTimeElement",))]
    #[allow(bad_style)]
    #[doc = "The `dateTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)\n\n*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
    #[allow(clippy::all)]
    pub fn date_time(&self) -> String {
        #[cfg(all(feature = "HtmlTimeElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_date_time_HTMLTimeElement(
                self_: <&HtmlTimeElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_date_time_HTMLTimeElement(
            self_: <&HtmlTimeElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlTimeElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_date_time_HTMLTimeElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlTimeElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_date_time_HTMLTimeElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlTimeElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlTimeElement {
    #[cfg(all(feature = "HtmlTimeElement",))]
    #[allow(bad_style)]
    #[doc = "The `dateTime` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)\n\n*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
    #[allow(clippy::all)]
    pub fn set_date_time(&self, date_time: &str) {
        #[cfg(all(feature = "HtmlTimeElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_date_time_HTMLTimeElement(
                self_: <&HtmlTimeElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                date_time: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_date_time_HTMLTimeElement(
            self_: <&HtmlTimeElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            date_time: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(date_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlTimeElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let date_time = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(date_time);
                __widl_f_set_date_time_HTMLTimeElement(self_, date_time)
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
pub static __WASM_BINDGEN_GENERATED_b6c5bd4094b88220: [u8; 347usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x19\x01\0\0\0\0\x03\0\0\x02\x0FHTMLTimeElement!__widl_instanceof_HTMLTimeElement\0\0\0\0\"__widl_f_date_time_HTMLTimeElement\0\0\0\x01\x0FHTMLTimeElement\x01\0\x01\x08dateTime\x01\x01\x05self_\x08dateTime\0\0\0&__widl_f_set_date_time_HTMLTimeElement\0\0\0\x01\x0FHTMLTimeElement\x01\0\x02\x08dateTime\x01\x02\x05self_\tdate_time\x08dateTime\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
