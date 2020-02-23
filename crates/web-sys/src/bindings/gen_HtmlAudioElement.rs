use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLAudioElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement)\n\n*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlAudioElement {
    obj: HtmlMediaElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlAudioElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlAudioElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlAudioElement {
        type Target = HtmlMediaElement;
        #[inline]
        fn deref(&self) -> &HtmlMediaElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlAudioElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlAudioElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlAudioElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlAudioElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlAudioElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlAudioElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlAudioElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlAudioElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlAudioElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlAudioElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlAudioElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlAudioElement {
            HtmlAudioElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlAudioElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlAudioElement> for HtmlAudioElement {
        #[inline]
        fn as_ref(&self) -> &HtmlAudioElement {
            self
        }
    }
    impl From<HtmlAudioElement> for JsValue {
        #[inline]
        fn from(obj: HtmlAudioElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlAudioElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLAudioElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLAudioElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLAudioElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlAudioElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlAudioElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlAudioElement> for HtmlMediaElement {
    #[inline]
    fn from(obj: HtmlAudioElement) -> HtmlMediaElement {
        use wasm_bindgen::JsCast;
        HtmlMediaElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlMediaElement> for HtmlAudioElement {
    #[inline]
    fn as_ref(&self) -> &HtmlMediaElement {
        use wasm_bindgen::JsCast;
        HtmlMediaElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAudioElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlAudioElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlAudioElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAudioElement> for Element {
    #[inline]
    fn from(obj: HtmlAudioElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlAudioElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAudioElement> for Node {
    #[inline]
    fn from(obj: HtmlAudioElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlAudioElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAudioElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlAudioElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlAudioElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAudioElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlAudioElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlAudioElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlAudioElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Audio() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <HtmlAudioElement as WasmDescribe>::describe();
}
impl HtmlAudioElement {
    #[cfg(all(feature = "HtmlAudioElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLAudioElement(..)` constructor, creating a new instance of `HTMLAudioElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)\n\n*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<HtmlAudioElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlAudioElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Audio() -> <HtmlAudioElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Audio(
        ) -> <HtmlAudioElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Audio() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlAudioElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlAudioElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_src_Audio() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <HtmlAudioElement as WasmDescribe>::describe();
}
impl HtmlAudioElement {
    #[cfg(all(feature = "HtmlAudioElement",))]
    #[allow(bad_style)]
    #[doc = "The `new HTMLAudioElement(..)` constructor, creating a new instance of `HTMLAudioElement`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)\n\n*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
    #[allow(clippy::all)]
    pub fn new_with_src(src: &str) -> Result<HtmlAudioElement, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlAudioElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_src_Audio(
                src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <HtmlAudioElement as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_src_Audio(
            src: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <HtmlAudioElement as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(src);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let src = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(src);
                __widl_f_new_with_src_Audio(src)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<HtmlAudioElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f4e3313de7726e1b: [u8; 252usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBA\0\0\0\0\0\x03\0\0\x02\x10HTMLAudioElement\"__widl_instanceof_HTMLAudioElement\0\0\0\0\x12__widl_f_new_Audio\x01\0\0\x01\x05Audio\0\x01\0\x03new\0\0\0\x1B__widl_f_new_with_src_Audio\x01\0\0\x01\x05Audio\0\x01\x01\x03src\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
