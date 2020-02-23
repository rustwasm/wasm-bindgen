use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlElement {
    obj: Element,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlElement {
        type Target = Element;
        #[inline]
        fn deref(&self) -> &Element {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlElement {
            HtmlElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlElement> for HtmlElement {
        #[inline]
        fn as_ref(&self) -> &HtmlElement {
            self
        }
    }
    impl From<HtmlElement> for JsValue {
        #[inline]
        fn from(obj: HtmlElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlElement> for Element {
    #[inline]
    fn from(obj: HtmlElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlElement> for Node {
    #[inline]
    fn from(obj: HtmlElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_blur_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `blur()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/blur)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn blur(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_blur_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_blur_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_blur_HTMLElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_click_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `click()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/click)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn click(&self) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_click_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_click_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_click_HTMLElement(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focus_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `focus()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/focus)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn focus(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focus_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focus_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focus_HTMLElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_title_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `title` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn title(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_title_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_title_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_title_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_title_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `title` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/title)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_title(&self, title: &str) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_title_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_title_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            title: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(title);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let title = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(title);
                __widl_f_set_title_HTMLElement(self_, title)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_lang_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `lang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn lang(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_lang_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_lang_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_lang_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_lang_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `lang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lang)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_lang(&self, lang: &str) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_lang_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                lang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_lang_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            lang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(lang);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let lang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(lang);
                __widl_f_set_lang_HTMLElement(self_, lang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dir_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `dir` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn dir(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dir_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dir_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dir_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_dir_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `dir` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dir)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_dir(&self, dir: &str) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_dir_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                dir: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_dir_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            dir: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(dir);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let dir = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(dir);
                __widl_f_set_dir_HTMLElement(self_, dir)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomStringMap", feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_dataset_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <DomStringMap as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "DomStringMap", feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `dataset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dataset)\n\n*This API requires the following crate features to be activated: `DomStringMap`, `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn dataset(&self) -> DomStringMap {
        #[cfg(all(feature = "DomStringMap", feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_dataset_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomStringMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_dataset_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomStringMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_dataset_HTMLElement(self_)
            };
            <DomStringMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_inner_text_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `innerText` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn inner_text(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_inner_text_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_inner_text_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_inner_text_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_inner_text_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `innerText` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/innerText)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_inner_text(&self, inner_text: &str) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_inner_text_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                inner_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_inner_text_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            inner_text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(inner_text);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let inner_text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(inner_text);
                __widl_f_set_inner_text_HTMLElement(self_, inner_text)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hidden_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `hidden` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn hidden(&self) -> bool {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hidden_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hidden_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hidden_HTMLElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hidden_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `hidden` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/hidden)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_hidden(&self, hidden: bool) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hidden_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hidden: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hidden_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hidden: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hidden);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hidden = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hidden);
                __widl_f_set_hidden_HTMLElement(self_, hidden)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_tab_index_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `tabIndex` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn tab_index(&self) -> i32 {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_tab_index_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_tab_index_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_tab_index_HTMLElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_tab_index_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `tabIndex` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/tabIndex)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_tab_index(&self, tab_index: i32) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_tab_index_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                tab_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_tab_index_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            tab_index: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(tab_index);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let tab_index = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(tab_index);
                __widl_f_set_tab_index_HTMLElement(self_, tab_index)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_access_key_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `accessKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn access_key(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_access_key_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_access_key_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_access_key_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_access_key_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `accessKey` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_access_key(&self, access_key: &str) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_access_key_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                access_key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_access_key_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            access_key: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(access_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let access_key = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(access_key);
                __widl_f_set_access_key_HTMLElement(self_, access_key)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_access_key_label_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `accessKeyLabel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKeyLabel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn access_key_label(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_access_key_label_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_access_key_label_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_access_key_label_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_draggable_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `draggable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn draggable(&self) -> bool {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_draggable_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_draggable_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_draggable_HTMLElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_draggable_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `draggable` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/draggable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_draggable(&self, draggable: bool) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_draggable_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                draggable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_draggable_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            draggable: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(draggable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let draggable = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(draggable);
                __widl_f_set_draggable_HTMLElement(self_, draggable)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_content_editable_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `contentEditable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn content_editable(&self) -> String {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_content_editable_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_content_editable_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_content_editable_HTMLElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_content_editable_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `contentEditable` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/contentEditable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_content_editable(&self, content_editable: &str) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_content_editable_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                content_editable: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_content_editable_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            content_editable: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(content_editable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let content_editable =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(content_editable);
                __widl_f_set_content_editable_HTMLElement(self_, content_editable)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_content_editable_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `isContentEditable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/isContentEditable)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn is_content_editable(&self) -> bool {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_content_editable_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_content_editable_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_is_content_editable_HTMLElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_spellcheck_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `spellcheck` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn spellcheck(&self) -> bool {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_spellcheck_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_spellcheck_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_spellcheck_HTMLElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_spellcheck_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `spellcheck` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/spellcheck)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_spellcheck(&self, spellcheck: bool) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_spellcheck_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                spellcheck: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_spellcheck_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            spellcheck: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(spellcheck);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let spellcheck = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(spellcheck);
                __widl_f_set_spellcheck_HTMLElement(self_, spellcheck)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssStyleDeclaration", feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_style_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <CssStyleDeclaration as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "CssStyleDeclaration", feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `style` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/style)\n\n*This API requires the following crate features to be activated: `CssStyleDeclaration`, `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn style(&self) -> CssStyleDeclaration {
        #[cfg(all(feature = "CssStyleDeclaration", feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_style_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_style_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_style_HTMLElement(self_)
            };
            <CssStyleDeclaration as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Element", feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_parent_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<Element> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "Element", feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `offsetParent` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetParent)\n\n*This API requires the following crate features to be activated: `Element`, `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn offset_parent(&self) -> Option<Element> {
        #[cfg(all(feature = "Element", feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_parent_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_parent_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_parent_HTMLElement(self_)
            };
            <Option<Element> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_top_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `offsetTop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetTop)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn offset_top(&self) -> i32 {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_top_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_top_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_top_HTMLElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_left_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `offsetLeft` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetLeft)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn offset_left(&self) -> i32 {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_left_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_left_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_left_HTMLElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_width_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `offsetWidth` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetWidth)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn offset_width(&self) -> i32 {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_width_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_width_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_width_HTMLElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_offset_height_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `offsetHeight` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetHeight)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn offset_height(&self) -> i32 {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_offset_height_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_offset_height_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <i32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_offset_height_HTMLElement(self_)
            };
            <i32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncopy_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncopy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oncopy(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncopy_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncopy_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncopy_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncopy_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncopy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncopy(&self, oncopy: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncopy_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncopy: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncopy_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncopy: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncopy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncopy =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncopy,
                    );
                __widl_f_set_oncopy_HTMLElement(self_, oncopy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncut_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncut` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oncut(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncut_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncut_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncut_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncut_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncut` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncut(&self, oncut: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncut_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncut: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncut_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncut: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncut);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncut =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncut,
                    );
                __widl_f_set_oncut_HTMLElement(self_, oncut)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpaste_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpaste` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpaste(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpaste_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpaste_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpaste_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpaste_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpaste` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpaste(&self, onpaste: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpaste_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpaste: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpaste_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpaste: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpaste);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpaste =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpaste,
                    );
                __widl_f_set_onpaste_HTMLElement(self_, onpaste)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onabort)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onabort)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onabort);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_HTMLElement(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onblur_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onblur` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onblur)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onblur(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onblur_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onblur_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onblur_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onblur_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onblur` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onblur)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onblur(&self, onblur: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onblur_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onblur: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onblur_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onblur: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onblur);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onblur =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onblur,
                    );
                __widl_f_set_onblur_HTMLElement(self_, onblur)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfocus_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onfocus` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onfocus)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onfocus(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfocus_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfocus_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onfocus_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfocus_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onfocus` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onfocus)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onfocus(&self, onfocus: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfocus_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfocus: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfocus_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfocus: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onfocus);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onfocus =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfocus,
                    );
                __widl_f_set_onfocus_HTMLElement(self_, onfocus)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onauxclick_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onauxclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onauxclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onauxclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onauxclick_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onauxclick_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onauxclick_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onauxclick_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onauxclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onauxclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onauxclick(&self, onauxclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onauxclick_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onauxclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onauxclick_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onauxclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onauxclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onauxclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onauxclick,
                    );
                __widl_f_set_onauxclick_HTMLElement(self_, onauxclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncanplay_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oncanplay(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncanplay_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncanplay_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncanplay_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncanplay_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncanplay(&self, oncanplay: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncanplay_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncanplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncanplay_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncanplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncanplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncanplay =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncanplay,
                    );
                __widl_f_set_oncanplay_HTMLElement(self_, oncanplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncanplaythrough_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplaythrough` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oncanplaythrough(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncanplaythrough_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncanplaythrough_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncanplaythrough_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncanplaythrough_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncanplaythrough` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncanplaythrough)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncanplaythrough(&self, oncanplaythrough: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncanplaythrough_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncanplaythrough : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncanplaythrough_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncanplaythrough : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(oncanplaythrough);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncanplaythrough =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncanplaythrough,
                    );
                __widl_f_set_oncanplaythrough_HTMLElement(self_, oncanplaythrough)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchange_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onchange(&self, onchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchange,
                    );
                __widl_f_set_onchange_HTMLElement(self_, onchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclick_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclick_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclick_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclick_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclick_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onclick(&self, onclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclick_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclick_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclick,
                    );
                __widl_f_set_onclick_HTMLElement(self_, onclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclose)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onclose)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_HTMLElement(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncontextmenu_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncontextmenu` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oncontextmenu(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncontextmenu_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncontextmenu_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncontextmenu_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncontextmenu_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oncontextmenu` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncontextmenu)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oncontextmenu(&self, oncontextmenu: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncontextmenu_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncontextmenu : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncontextmenu_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncontextmenu: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncontextmenu);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncontextmenu =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncontextmenu,
                    );
                __widl_f_set_oncontextmenu_HTMLElement(self_, oncontextmenu)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondblclick_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondblclick` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondblclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondblclick(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondblclick_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondblclick_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondblclick_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondblclick_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondblclick` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondblclick)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondblclick(&self, ondblclick: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondblclick_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondblclick : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondblclick_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondblclick: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondblclick);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondblclick =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondblclick,
                    );
                __widl_f_set_ondblclick_HTMLElement(self_, ondblclick)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrag_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrag` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrag)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondrag(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrag_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrag_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrag_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrag_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrag` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrag)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondrag(&self, ondrag: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrag_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrag: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrag_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrag: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrag =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrag,
                    );
                __widl_f_set_ondrag_HTMLElement(self_, ondrag)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondragend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragend(&self, ondragend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragend,
                    );
                __widl_f_set_ondragend_HTMLElement(self_, ondragend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragenter_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondragenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragenter_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragenter_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragenter_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragenter_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragenter(&self, ondragenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragenter_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragenter_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragenter,
                    );
                __widl_f_set_ondragenter_HTMLElement(self_, ondragenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragexit_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragexit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragexit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondragexit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragexit_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragexit_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragexit_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragexit_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragexit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragexit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragexit(&self, ondragexit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragexit_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragexit : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragexit_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragexit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragexit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragexit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragexit,
                    );
                __widl_f_set_ondragexit_HTMLElement(self_, ondragexit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragleave_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondragleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragleave_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragleave_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragleave_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragleave_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragleave(&self, ondragleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragleave_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragleave_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragleave: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragleave,
                    );
                __widl_f_set_ondragleave_HTMLElement(self_, ondragleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragover_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondragover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragover_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragover_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragover_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragover_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragover(&self, ondragover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragover_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragover_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragover,
                    );
                __widl_f_set_ondragover_HTMLElement(self_, ondragover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondragstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondragstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondragstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondragstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondragstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondragstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondragstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondragstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondragstart(&self, ondragstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondragstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondragstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondragstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondragstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondragstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondragstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondragstart,
                    );
                __widl_f_set_ondragstart_HTMLElement(self_, ondragstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondrop_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrop)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondrop(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondrop_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondrop_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondrop_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondrop_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondrop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondrop)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondrop(&self, ondrop: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondrop_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondrop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondrop_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondrop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ondrop);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondrop =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondrop,
                    );
                __widl_f_set_ondrop_HTMLElement(self_, ondrop)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondurationchange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondurationchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ondurationchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondurationchange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondurationchange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondurationchange_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondurationchange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ondurationchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ondurationchange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ondurationchange(&self, ondurationchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondurationchange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondurationchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondurationchange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondurationchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ondurationchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondurationchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondurationchange,
                    );
                __widl_f_set_ondurationchange_HTMLElement(self_, ondurationchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onemptied_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onemptied` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onemptied)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onemptied(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onemptied_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onemptied_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onemptied_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onemptied_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onemptied` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onemptied)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onemptied(&self, onemptied: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onemptied_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onemptied: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onemptied_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onemptied: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onemptied);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onemptied =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onemptied,
                    );
                __widl_f_set_onemptied_HTMLElement(self_, onemptied)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onended)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onended)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_HTMLElement(self_, onended)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninput_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninput` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninput)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oninput(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninput_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninput_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oninput_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninput_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninput` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninput)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oninput(&self, oninput: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninput_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninput: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninput_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninput: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninput);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oninput =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninput,
                    );
                __widl_f_set_oninput_HTMLElement(self_, oninput)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oninvalid_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninvalid` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninvalid)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn oninvalid(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oninvalid_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oninvalid_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oninvalid_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oninvalid_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `oninvalid` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oninvalid)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_oninvalid(&self, oninvalid: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oninvalid_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oninvalid: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oninvalid_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oninvalid: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oninvalid);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oninvalid =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oninvalid,
                    );
                __widl_f_set_oninvalid_HTMLElement(self_, oninvalid)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeydown_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeydown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeydown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onkeydown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeydown_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeydown_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeydown_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeydown_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeydown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeydown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onkeydown(&self, onkeydown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeydown_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeydown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeydown_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeydown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeydown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeydown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeydown,
                    );
                __widl_f_set_onkeydown_HTMLElement(self_, onkeydown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeypress_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeypress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeypress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onkeypress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeypress_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeypress_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeypress_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeypress_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeypress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeypress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onkeypress(&self, onkeypress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeypress_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeypress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeypress_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeypress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeypress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeypress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeypress,
                    );
                __widl_f_set_onkeypress_HTMLElement(self_, onkeypress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeyup_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeyup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeyup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onkeyup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeyup_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeyup_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeyup_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeyup_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onkeyup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onkeyup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onkeyup(&self, onkeyup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeyup_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeyup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeyup_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeyup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onkeyup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeyup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeyup,
                    );
                __widl_f_set_onkeyup_HTMLElement(self_, onkeyup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onload_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onload)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onload_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onload_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onload_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onload_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onload)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onload(&self, onload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onload_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onload_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onload,
                    );
                __widl_f_set_onload_HTMLElement(self_, onload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadeddata_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadeddata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onloadeddata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadeddata_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadeddata_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadeddata_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadeddata_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadeddata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadeddata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadeddata(&self, onloadeddata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadeddata_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadeddata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadeddata_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadeddata: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadeddata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadeddata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadeddata,
                    );
                __widl_f_set_onloadeddata_HTMLElement(self_, onloadeddata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadedmetadata_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadedmetadata` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onloadedmetadata(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadedmetadata_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadedmetadata_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadedmetadata_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadedmetadata_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadedmetadata` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadedmetadata)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadedmetadata(&self, onloadedmetadata: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadedmetadata_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadedmetadata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadedmetadata_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadedmetadata : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onloadedmetadata);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadedmetadata =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadedmetadata,
                    );
                __widl_f_set_onloadedmetadata_HTMLElement(self_, onloadedmetadata)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onloadend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadend(&self, onloadend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadend,
                    );
                __widl_f_set_onloadend_HTMLElement(self_, onloadend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onloadstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onloadstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onloadstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onloadstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onloadstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onloadstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onloadstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onloadstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onloadstart(&self, onloadstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onloadstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onloadstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onloadstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onloadstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onloadstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onloadstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onloadstart,
                    );
                __widl_f_set_onloadstart_HTMLElement(self_, onloadstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmousedown_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousedown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousedown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmousedown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmousedown_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmousedown_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmousedown_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmousedown_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousedown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousedown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmousedown(&self, onmousedown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmousedown_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmousedown : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmousedown_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmousedown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmousedown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmousedown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmousedown,
                    );
                __widl_f_set_onmousedown_HTMLElement(self_, onmousedown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseenter_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseenter_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseenter_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseenter_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseenter_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseenter(&self, onmouseenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseenter_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseenter_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseenter,
                    );
                __widl_f_set_onmouseenter_HTMLElement(self_, onmouseenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseleave_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseleave_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseleave_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseleave_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseleave_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseleave(&self, onmouseleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseleave_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseleave_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseleave: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseleave,
                    );
                __widl_f_set_onmouseleave_HTMLElement(self_, onmouseleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmousemove_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousemove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousemove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmousemove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmousemove_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmousemove_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmousemove_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmousemove_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmousemove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmousemove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmousemove(&self, onmousemove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmousemove_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmousemove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmousemove_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmousemove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmousemove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmousemove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmousemove,
                    );
                __widl_f_set_onmousemove_HTMLElement(self_, onmousemove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseout_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseout_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseout_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseout_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseout_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseout(&self, onmouseout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseout_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseout : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseout_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseout,
                    );
                __widl_f_set_onmouseout_HTMLElement(self_, onmouseout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseover_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseover_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseover_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseover_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseover_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseover(&self, onmouseover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseover_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseover_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseover,
                    );
                __widl_f_set_onmouseover_HTMLElement(self_, onmouseover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmouseup_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onmouseup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmouseup_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmouseup_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmouseup_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmouseup_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmouseup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onmouseup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmouseup(&self, onmouseup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmouseup_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmouseup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmouseup_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmouseup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmouseup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmouseup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmouseup,
                    );
                __widl_f_set_onmouseup_HTMLElement(self_, onmouseup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwheel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwheel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwheel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onwheel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwheel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwheel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwheel_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwheel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwheel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwheel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwheel(&self, onwheel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwheel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwheel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwheel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwheel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwheel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwheel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwheel,
                    );
                __widl_f_set_onwheel_HTMLElement(self_, onwheel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpause_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpause)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpause(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpause_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpause_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpause_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpause_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpause` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpause)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpause(&self, onpause: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpause_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpause_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpause: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpause);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpause =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpause,
                    );
                __widl_f_set_onpause_HTMLElement(self_, onpause)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onplay_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplay` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onplay(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onplay_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onplay_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onplay_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onplay_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplay` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplay)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onplay(&self, onplay: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onplay_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onplay_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onplay: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onplay);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onplay =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onplay,
                    );
                __widl_f_set_onplay_HTMLElement(self_, onplay)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onplaying_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplaying` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplaying)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onplaying(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onplaying_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onplaying_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onplaying_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onplaying_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onplaying` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onplaying)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onplaying(&self, onplaying: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onplaying_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onplaying: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onplaying_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onplaying: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onplaying);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onplaying =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onplaying,
                    );
                __widl_f_set_onplaying_HTMLElement(self_, onplaying)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onprogress_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onprogress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onprogress(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onprogress_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onprogress_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onprogress_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onprogress_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onprogress` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onprogress)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onprogress(&self, onprogress: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onprogress_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onprogress : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onprogress_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onprogress: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onprogress);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onprogress =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onprogress,
                    );
                __widl_f_set_onprogress_HTMLElement(self_, onprogress)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onratechange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onratechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onratechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onratechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onratechange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onratechange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onratechange_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onratechange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onratechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onratechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onratechange(&self, onratechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onratechange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onratechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onratechange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onratechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onratechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onratechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onratechange,
                    );
                __widl_f_set_onratechange_HTMLElement(self_, onratechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onreset_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onreset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onreset)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onreset(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onreset_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onreset_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onreset_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onreset_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onreset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onreset)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onreset(&self, onreset: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onreset_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onreset: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onreset_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onreset: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onreset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onreset =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onreset,
                    );
                __widl_f_set_onreset_HTMLElement(self_, onreset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onresize_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onresize` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onresize)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onresize(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onresize_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onresize_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onresize_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onresize_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onresize` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onresize)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onresize(&self, onresize: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onresize_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onresize: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onresize_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onresize: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onresize);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onresize =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onresize,
                    );
                __widl_f_set_onresize_HTMLElement(self_, onresize)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onscroll_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onscroll` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onscroll)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onscroll(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onscroll_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onscroll_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onscroll_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onscroll_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onscroll` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onscroll)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onscroll(&self, onscroll: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onscroll_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onscroll: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onscroll_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onscroll: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onscroll);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onscroll =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onscroll,
                    );
                __widl_f_set_onscroll_HTMLElement(self_, onscroll)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onseeked_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeked)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onseeked(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onseeked_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onseeked_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onseeked_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onseeked_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeked)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onseeked(&self, onseeked: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onseeked_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onseeked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onseeked_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onseeked: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onseeked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onseeked =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onseeked,
                    );
                __widl_f_set_onseeked_HTMLElement(self_, onseeked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onseeking_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeking` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeking)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onseeking(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onseeking_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onseeking_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onseeking_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onseeking_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onseeking` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onseeking)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onseeking(&self, onseeking: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onseeking_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onseeking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onseeking_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onseeking: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onseeking);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onseeking =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onseeking,
                    );
                __widl_f_set_onseeking_HTMLElement(self_, onseeking)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselect_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselect)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onselect(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselect_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselect_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselect_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselect_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselect)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onselect(&self, onselect: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselect_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselect_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onselect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselect =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselect,
                    );
                __widl_f_set_onselect_HTMLElement(self_, onselect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onshow_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onshow)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onshow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onshow_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onshow_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onshow_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onshow_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onshow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onshow)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onshow(&self, onshow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onshow_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onshow_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onshow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onshow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onshow,
                    );
                __widl_f_set_onshow_HTMLElement(self_, onshow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstalled_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstalled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onstalled)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onstalled(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstalled_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstalled_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstalled_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstalled_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstalled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onstalled)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onstalled(&self, onstalled: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstalled_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstalled: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstalled_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstalled: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstalled);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstalled =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstalled,
                    );
                __widl_f_set_onstalled_HTMLElement(self_, onstalled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsubmit_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsubmit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsubmit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onsubmit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsubmit_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsubmit_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsubmit_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsubmit_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsubmit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsubmit)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onsubmit(&self, onsubmit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsubmit_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsubmit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsubmit_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsubmit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsubmit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsubmit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsubmit,
                    );
                __widl_f_set_onsubmit_HTMLElement(self_, onsubmit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsuspend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsuspend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsuspend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onsuspend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsuspend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsuspend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsuspend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsuspend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onsuspend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onsuspend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onsuspend(&self, onsuspend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsuspend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsuspend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsuspend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsuspend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsuspend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsuspend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsuspend,
                    );
                __widl_f_set_onsuspend_HTMLElement(self_, onsuspend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontimeupdate_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeupdate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontimeupdate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontimeupdate_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontimeupdate_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontimeupdate_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontimeupdate_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontimeupdate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontimeupdate)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontimeupdate(&self, ontimeupdate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontimeupdate_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontimeupdate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontimeupdate_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontimeupdate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontimeupdate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontimeupdate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontimeupdate,
                    );
                __widl_f_set_ontimeupdate_HTMLElement(self_, ontimeupdate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onvolumechange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onvolumechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onvolumechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onvolumechange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onvolumechange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onvolumechange_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onvolumechange_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onvolumechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onvolumechange)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onvolumechange(&self, onvolumechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onvolumechange_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onvolumechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onvolumechange_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onvolumechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onvolumechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onvolumechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onvolumechange,
                    );
                __widl_f_set_onvolumechange_HTMLElement(self_, onvolumechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwaiting_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwaiting` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwaiting)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onwaiting(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwaiting_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwaiting_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwaiting_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwaiting_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwaiting` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwaiting)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwaiting(&self, onwaiting: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwaiting_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwaiting: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwaiting_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwaiting: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwaiting);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwaiting =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwaiting,
                    );
                __widl_f_set_onwaiting_HTMLElement(self_, onwaiting)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onselectstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselectstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselectstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onselectstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onselectstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onselectstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onselectstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onselectstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onselectstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onselectstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onselectstart(&self, onselectstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onselectstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onselectstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onselectstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onselectstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onselectstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onselectstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onselectstart,
                    );
                __widl_f_set_onselectstart_HTMLElement(self_, onselectstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontoggle_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontoggle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontoggle)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontoggle(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontoggle_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontoggle_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontoggle_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontoggle_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontoggle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontoggle)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontoggle(&self, ontoggle: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontoggle_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontoggle: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontoggle_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontoggle: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontoggle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontoggle =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontoggle,
                    );
                __widl_f_set_ontoggle_HTMLElement(self_, ontoggle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointercancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointercancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointercancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointercancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointercancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointercancel_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointercancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointercancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointercancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointercancel(&self, onpointercancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointercancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointercancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointercancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointercancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointercancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointercancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointercancel,
                    );
                __widl_f_set_onpointercancel_HTMLElement(self_, onpointercancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerdown_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerdown` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerdown(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerdown_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerdown_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerdown_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerdown_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerdown` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerdown)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerdown(&self, onpointerdown: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerdown_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerdown : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerdown_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerdown: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerdown);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerdown =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerdown,
                    );
                __widl_f_set_onpointerdown_HTMLElement(self_, onpointerdown)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerup_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerup(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerup_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerup_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerup_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerup_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerup)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerup(&self, onpointerup: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerup_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerup : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerup_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerup: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerup =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerup,
                    );
                __widl_f_set_onpointerup_HTMLElement(self_, onpointerup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointermove_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointermove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointermove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointermove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointermove_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointermove_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointermove_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointermove_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointermove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointermove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointermove(&self, onpointermove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointermove_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointermove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointermove_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointermove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointermove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointermove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointermove,
                    );
                __widl_f_set_onpointermove_HTMLElement(self_, onpointermove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerout_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerout` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerout(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerout_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerout_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerout_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerout_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerout` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerout)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerout(&self, onpointerout: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerout_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerout : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerout_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerout: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerout);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerout =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerout,
                    );
                __widl_f_set_onpointerout_HTMLElement(self_, onpointerout)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerover_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerover` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerover(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerover_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerover_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerover_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerover_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerover` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerover)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerover(&self, onpointerover: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerover_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerover : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerover_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerover: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpointerover);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerover =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerover,
                    );
                __widl_f_set_onpointerover_HTMLElement(self_, onpointerover)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerenter_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerenter_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerenter_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerenter_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerenter_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerenter)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerenter(&self, onpointerenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerenter_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerenter_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerenter : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerenter,
                    );
                __widl_f_set_onpointerenter_HTMLElement(self_, onpointerenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpointerleave_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerleave` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onpointerleave(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpointerleave_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpointerleave_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpointerleave_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpointerleave_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpointerleave` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpointerleave)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpointerleave(&self, onpointerleave: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpointerleave_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpointerleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpointerleave_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpointerleave : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onpointerleave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpointerleave =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpointerleave,
                    );
                __widl_f_set_onpointerleave_HTMLElement(self_, onpointerleave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ongotpointercapture_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ongotpointercapture` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ongotpointercapture(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ongotpointercapture_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ongotpointercapture_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ongotpointercapture_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ongotpointercapture_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ongotpointercapture` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ongotpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ongotpointercapture(&self, ongotpointercapture: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ongotpointercapture_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ongotpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ongotpointercapture_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ongotpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ongotpointercapture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ongotpointercapture =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ongotpointercapture,
                    );
                __widl_f_set_ongotpointercapture_HTMLElement(self_, ongotpointercapture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onlostpointercapture_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlostpointercapture` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onlostpointercapture(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onlostpointercapture_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onlostpointercapture_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onlostpointercapture_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onlostpointercapture_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlostpointercapture` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onlostpointercapture)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onlostpointercapture(&self, onlostpointercapture: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onlostpointercapture_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onlostpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onlostpointercapture_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onlostpointercapture : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onlostpointercapture);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onlostpointercapture =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onlostpointercapture,
                    );
                __widl_f_set_onlostpointercapture_HTMLElement(self_, onlostpointercapture)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationcancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationcancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationcancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationcancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationcancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationcancel_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationcancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationcancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationcancel(&self, onanimationcancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationcancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationcancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationcancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationcancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationcancel,
                    );
                __widl_f_set_onanimationcancel_HTMLElement(self_, onanimationcancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationend(&self, onanimationend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationend,
                    );
                __widl_f_set_onanimationend_HTMLElement(self_, onanimationend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationiteration_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationiteration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationiteration(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationiteration_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationiteration_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationiteration_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationiteration_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationiteration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationiteration(&self, onanimationiteration: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationiteration_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationiteration_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationiteration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationiteration =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationiteration,
                    );
                __widl_f_set_onanimationiteration_HTMLElement(self_, onanimationiteration)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onanimationstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onanimationstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onanimationstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onanimationstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onanimationstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onanimationstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onanimationstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onanimationstart(&self, onanimationstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onanimationstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onanimationstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onanimationstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onanimationstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onanimationstart,
                    );
                __widl_f_set_onanimationstart_HTMLElement(self_, onanimationstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitioncancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitioncancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitioncancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitioncancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitioncancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitioncancel_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitioncancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitioncancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitioncancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitioncancel(&self, ontransitioncancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitioncancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitioncancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitioncancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitioncancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitioncancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitioncancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitioncancel,
                    );
                __widl_f_set_ontransitioncancel_HTMLElement(self_, ontransitioncancel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitionend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionend(&self, ontransitionend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionend,
                    );
                __widl_f_set_ontransitionend_HTMLElement(self_, ontransitionend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionrun_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionrun` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitionrun(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionrun_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionrun_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionrun_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionrun_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionrun` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionrun)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionrun(&self, ontransitionrun: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionrun_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionrun : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionrun_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionrun : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionrun);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionrun =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionrun,
                    );
                __widl_f_set_ontransitionrun_HTMLElement(self_, ontransitionrun)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontransitionstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontransitionstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontransitionstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontransitionstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontransitionstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontransitionstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontransitionstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontransitionstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontransitionstart(&self, ontransitionstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontransitionstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontransitionstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontransitionstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontransitionstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ontransitionstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontransitionstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontransitionstart,
                    );
                __widl_f_set_ontransitionstart_HTMLElement(self_, ontransitionstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationend(&self, onwebkitanimationend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationend,
                    );
                __widl_f_set_onwebkitanimationend_HTMLElement(self_, onwebkitanimationend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationiteration_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationiteration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationiteration(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationiteration_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationiteration_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationiteration_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationiteration_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationiteration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationiteration)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationiteration(
        &self,
        onwebkitanimationiteration: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationiteration_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationiteration_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationiteration : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationiteration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationiteration =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationiteration,
                    );
                __widl_f_set_onwebkitanimationiteration_HTMLElement(
                    self_,
                    onwebkitanimationiteration,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkitanimationstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkitanimationstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkitanimationstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkitanimationstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkitanimationstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkitanimationstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkitanimationstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkitanimationstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkitanimationstart(&self, onwebkitanimationstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkitanimationstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkitanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkitanimationstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkitanimationstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkitanimationstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkitanimationstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkitanimationstart,
                    );
                __widl_f_set_onwebkitanimationstart_HTMLElement(self_, onwebkitanimationstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwebkittransitionend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkittransitionend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onwebkittransitionend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwebkittransitionend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwebkittransitionend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwebkittransitionend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwebkittransitionend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onwebkittransitionend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onwebkittransitionend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onwebkittransitionend(&self, onwebkittransitionend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwebkittransitionend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwebkittransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwebkittransitionend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwebkittransitionend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onwebkittransitionend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwebkittransitionend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwebkittransitionend,
                    );
                __widl_f_set_onwebkittransitionend_HTMLElement(self_, onwebkittransitionend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onerror)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onerror)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_HTMLElement(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchstart_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchstart_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchstart)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchstart(&self, ontouchstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchstart_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchstart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchstart_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchstart,
                    );
                __widl_f_set_ontouchstart_HTMLElement(self_, ontouchstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchend_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchend_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchend)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchend(&self, ontouchend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchend_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchend_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchend,
                    );
                __widl_f_set_ontouchend_HTMLElement(self_, ontouchend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchmove_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchmove` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchmove(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchmove_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchmove_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchmove_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchmove_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchmove` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchmove)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchmove(&self, ontouchmove: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchmove_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchmove : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchmove_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchmove: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchmove);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchmove =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchmove,
                    );
                __widl_f_set_ontouchmove_HTMLElement(self_, ontouchmove)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ontouchcancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchcancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn ontouchcancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ontouchcancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ontouchcancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ontouchcancel_HTMLElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ontouchcancel_HTMLElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlElement {
    #[cfg(all(feature = "HtmlElement",))]
    #[allow(bad_style)]
    #[doc = "The `ontouchcancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/ontouchcancel)\n\n*This API requires the following crate features to be activated: `HtmlElement`*"]
    #[allow(clippy::all)]
    pub fn set_ontouchcancel(&self, ontouchcancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ontouchcancel_HTMLElement(
                self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ontouchcancel : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ontouchcancel_HTMLElement(
            self_: <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ontouchcancel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ontouchcancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ontouchcancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ontouchcancel,
                    );
                __widl_f_set_ontouchcancel_HTMLElement(self_, ontouchcancel)
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
pub static __WASM_BINDGEN_GENERATED_42f62b01efe2bfe1: [u8; 19990usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD4M\0\0\0\0\xD5\x01\0\0\x02\x0BHTMLElement\x1D__widl_instanceof_HTMLElement\0\0\0\0\x19__widl_f_blur_HTMLElement\x01\0\0\x01\x0BHTMLElement\x01\0\0\x01\x01\x05self_\x04blur\0\0\0\x1A__widl_f_click_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\0\x01\x01\x05self_\x05click\0\0\0\x1A__widl_f_focus_HTMLElement\x01\0\0\x01\x0BHTMLElement\x01\0\0\x01\x01\x05self_\x05focus\0\0\0\x1A__widl_f_title_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x05title\x01\x01\x05self_\x05title\0\0\0\x1E__widl_f_set_title_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x05title\x01\x02\x05self_\x05title\x05title\0\0\0\x19__widl_f_lang_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x04lang\x01\x01\x05self_\x04lang\0\0\0\x1D__widl_f_set_lang_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x04lang\x01\x02\x05self_\x04lang\x04lang\0\0\0\x18__widl_f_dir_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x03dir\x01\x01\x05self_\x03dir\0\0\0\x1C__widl_f_set_dir_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x03dir\x01\x02\x05self_\x03dir\x03dir\0\0\0\x1C__widl_f_dataset_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07dataset\x01\x01\x05self_\x07dataset\0\0\0\x1F__widl_f_inner_text_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tinnerText\x01\x01\x05self_\tinnerText\0\0\0#__widl_f_set_inner_text_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tinnerText\x01\x02\x05self_\ninner_text\tinnerText\0\0\0\x1B__widl_f_hidden_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06hidden\x01\x01\x05self_\x06hidden\0\0\0\x1F__widl_f_set_hidden_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06hidden\x01\x02\x05self_\x06hidden\x06hidden\0\0\0\x1E__widl_f_tab_index_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08tabIndex\x01\x01\x05self_\x08tabIndex\0\0\0\"__widl_f_set_tab_index_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08tabIndex\x01\x02\x05self_\ttab_index\x08tabIndex\0\0\0\x1F__widl_f_access_key_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\taccessKey\x01\x01\x05self_\taccessKey\0\0\0#__widl_f_set_access_key_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\taccessKey\x01\x02\x05self_\naccess_key\taccessKey\0\0\0%__widl_f_access_key_label_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0EaccessKeyLabel\x01\x01\x05self_\x0EaccessKeyLabel\0\0\0\x1E__widl_f_draggable_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tdraggable\x01\x01\x05self_\tdraggable\0\0\0\"__widl_f_set_draggable_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tdraggable\x01\x02\x05self_\tdraggable\tdraggable\0\0\0%__widl_f_content_editable_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0FcontentEditable\x01\x01\x05self_\x0FcontentEditable\0\0\0)__widl_f_set_content_editable_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0FcontentEditable\x01\x02\x05self_\x10content_editable\x0FcontentEditable\0\0\0(__widl_f_is_content_editable_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x11isContentEditable\x01\x01\x05self_\x11isContentEditable\0\0\0\x1F__widl_f_spellcheck_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nspellcheck\x01\x01\x05self_\nspellcheck\0\0\0#__widl_f_set_spellcheck_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nspellcheck\x01\x02\x05self_\nspellcheck\nspellcheck\0\0\0\x1A__widl_f_style_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x05style\x01\x01\x05self_\x05style\0\0\0\"__widl_f_offset_parent_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0CoffsetParent\x01\x01\x05self_\x0CoffsetParent\0\0\0\x1F__widl_f_offset_top_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\toffsetTop\x01\x01\x05self_\toffsetTop\0\0\0 __widl_f_offset_left_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\noffsetLeft\x01\x01\x05self_\noffsetLeft\0\0\0!__widl_f_offset_width_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0BoffsetWidth\x01\x01\x05self_\x0BoffsetWidth\0\0\0\"__widl_f_offset_height_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0CoffsetHeight\x01\x01\x05self_\x0CoffsetHeight\0\0\0\x1B__widl_f_oncopy_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06oncopy\x01\x01\x05self_\x06oncopy\0\0\0\x1F__widl_f_set_oncopy_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06oncopy\x01\x02\x05self_\x06oncopy\x06oncopy\0\0\0\x1A__widl_f_oncut_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x05oncut\x01\x01\x05self_\x05oncut\0\0\0\x1E__widl_f_set_oncut_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x05oncut\x01\x02\x05self_\x05oncut\x05oncut\0\0\0\x1C__widl_f_onpaste_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onpaste\x01\x01\x05self_\x07onpaste\0\0\0 __widl_f_set_onpaste_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onpaste\x01\x02\x05self_\x07onpaste\x07onpaste\0\0\0\x1C__widl_f_onabort_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0 __widl_f_set_onabort_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x1B__widl_f_onblur_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06onblur\x01\x01\x05self_\x06onblur\0\0\0\x1F__widl_f_set_onblur_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06onblur\x01\x02\x05self_\x06onblur\x06onblur\0\0\0\x1C__widl_f_onfocus_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onfocus\x01\x01\x05self_\x07onfocus\0\0\0 __widl_f_set_onfocus_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onfocus\x01\x02\x05self_\x07onfocus\x07onfocus\0\0\0\x1F__widl_f_onauxclick_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nonauxclick\x01\x01\x05self_\nonauxclick\0\0\0#__widl_f_set_onauxclick_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nonauxclick\x01\x02\x05self_\nonauxclick\nonauxclick\0\0\0\x1E__widl_f_oncanplay_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\toncanplay\x01\x01\x05self_\toncanplay\0\0\0\"__widl_f_set_oncanplay_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\toncanplay\x01\x02\x05self_\toncanplay\toncanplay\0\0\0%__widl_f_oncanplaythrough_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x10oncanplaythrough\x01\x01\x05self_\x10oncanplaythrough\0\0\0)__widl_f_set_oncanplaythrough_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x10oncanplaythrough\x01\x02\x05self_\x10oncanplaythrough\x10oncanplaythrough\0\0\0\x1D__widl_f_onchange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08onchange\x01\x01\x05self_\x08onchange\0\0\0!__widl_f_set_onchange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08onchange\x01\x02\x05self_\x08onchange\x08onchange\0\0\0\x1C__widl_f_onclick_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onclick\x01\x01\x05self_\x07onclick\0\0\0 __widl_f_set_onclick_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onclick\x01\x02\x05self_\x07onclick\x07onclick\0\0\0\x1C__widl_f_onclose_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0 __widl_f_set_onclose_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0\"__widl_f_oncontextmenu_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\roncontextmenu\x01\x01\x05self_\roncontextmenu\0\0\0&__widl_f_set_oncontextmenu_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\roncontextmenu\x01\x02\x05self_\roncontextmenu\roncontextmenu\0\0\0\x1F__widl_f_ondblclick_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nondblclick\x01\x01\x05self_\nondblclick\0\0\0#__widl_f_set_ondblclick_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nondblclick\x01\x02\x05self_\nondblclick\nondblclick\0\0\0\x1B__widl_f_ondrag_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06ondrag\x01\x01\x05self_\x06ondrag\0\0\0\x1F__widl_f_set_ondrag_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06ondrag\x01\x02\x05self_\x06ondrag\x06ondrag\0\0\0\x1E__widl_f_ondragend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tondragend\x01\x01\x05self_\tondragend\0\0\0\"__widl_f_set_ondragend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tondragend\x01\x02\x05self_\tondragend\tondragend\0\0\0 __widl_f_ondragenter_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bondragenter\x01\x01\x05self_\x0Bondragenter\0\0\0$__widl_f_set_ondragenter_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bondragenter\x01\x02\x05self_\x0Bondragenter\x0Bondragenter\0\0\0\x1F__widl_f_ondragexit_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nondragexit\x01\x01\x05self_\nondragexit\0\0\0#__widl_f_set_ondragexit_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nondragexit\x01\x02\x05self_\nondragexit\nondragexit\0\0\0 __widl_f_ondragleave_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bondragleave\x01\x01\x05self_\x0Bondragleave\0\0\0$__widl_f_set_ondragleave_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bondragleave\x01\x02\x05self_\x0Bondragleave\x0Bondragleave\0\0\0\x1F__widl_f_ondragover_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nondragover\x01\x01\x05self_\nondragover\0\0\0#__widl_f_set_ondragover_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nondragover\x01\x02\x05self_\nondragover\nondragover\0\0\0 __widl_f_ondragstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bondragstart\x01\x01\x05self_\x0Bondragstart\0\0\0$__widl_f_set_ondragstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bondragstart\x01\x02\x05self_\x0Bondragstart\x0Bondragstart\0\0\0\x1B__widl_f_ondrop_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06ondrop\x01\x01\x05self_\x06ondrop\0\0\0\x1F__widl_f_set_ondrop_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06ondrop\x01\x02\x05self_\x06ondrop\x06ondrop\0\0\0%__widl_f_ondurationchange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x10ondurationchange\x01\x01\x05self_\x10ondurationchange\0\0\0)__widl_f_set_ondurationchange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x10ondurationchange\x01\x02\x05self_\x10ondurationchange\x10ondurationchange\0\0\0\x1E__widl_f_onemptied_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonemptied\x01\x01\x05self_\tonemptied\0\0\0\"__widl_f_set_onemptied_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonemptied\x01\x02\x05self_\tonemptied\tonemptied\0\0\0\x1C__widl_f_onended_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0 __widl_f_set_onended_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\x1C__widl_f_oninput_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07oninput\x01\x01\x05self_\x07oninput\0\0\0 __widl_f_set_oninput_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07oninput\x01\x02\x05self_\x07oninput\x07oninput\0\0\0\x1E__widl_f_oninvalid_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\toninvalid\x01\x01\x05self_\toninvalid\0\0\0\"__widl_f_set_oninvalid_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\toninvalid\x01\x02\x05self_\toninvalid\toninvalid\0\0\0\x1E__widl_f_onkeydown_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonkeydown\x01\x01\x05self_\tonkeydown\0\0\0\"__widl_f_set_onkeydown_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonkeydown\x01\x02\x05self_\tonkeydown\tonkeydown\0\0\0\x1F__widl_f_onkeypress_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nonkeypress\x01\x01\x05self_\nonkeypress\0\0\0#__widl_f_set_onkeypress_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nonkeypress\x01\x02\x05self_\nonkeypress\nonkeypress\0\0\0\x1C__widl_f_onkeyup_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onkeyup\x01\x01\x05self_\x07onkeyup\0\0\0 __widl_f_set_onkeyup_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onkeyup\x01\x02\x05self_\x07onkeyup\x07onkeyup\0\0\0\x1B__widl_f_onload_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06onload\x01\x01\x05self_\x06onload\0\0\0\x1F__widl_f_set_onload_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06onload\x01\x02\x05self_\x06onload\x06onload\0\0\0!__widl_f_onloadeddata_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Conloadeddata\x01\x01\x05self_\x0Conloadeddata\0\0\0%__widl_f_set_onloadeddata_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Conloadeddata\x01\x02\x05self_\x0Conloadeddata\x0Conloadeddata\0\0\0%__widl_f_onloadedmetadata_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x10onloadedmetadata\x01\x01\x05self_\x10onloadedmetadata\0\0\0)__widl_f_set_onloadedmetadata_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x10onloadedmetadata\x01\x02\x05self_\x10onloadedmetadata\x10onloadedmetadata\0\0\0\x1E__widl_f_onloadend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonloadend\x01\x01\x05self_\tonloadend\0\0\0\"__widl_f_set_onloadend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonloadend\x01\x02\x05self_\tonloadend\tonloadend\0\0\0 __widl_f_onloadstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bonloadstart\x01\x01\x05self_\x0Bonloadstart\0\0\0$__widl_f_set_onloadstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bonloadstart\x01\x02\x05self_\x0Bonloadstart\x0Bonloadstart\0\0\0 __widl_f_onmousedown_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bonmousedown\x01\x01\x05self_\x0Bonmousedown\0\0\0$__widl_f_set_onmousedown_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bonmousedown\x01\x02\x05self_\x0Bonmousedown\x0Bonmousedown\0\0\0!__widl_f_onmouseenter_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Conmouseenter\x01\x01\x05self_\x0Conmouseenter\0\0\0%__widl_f_set_onmouseenter_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Conmouseenter\x01\x02\x05self_\x0Conmouseenter\x0Conmouseenter\0\0\0!__widl_f_onmouseleave_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Conmouseleave\x01\x01\x05self_\x0Conmouseleave\0\0\0%__widl_f_set_onmouseleave_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Conmouseleave\x01\x02\x05self_\x0Conmouseleave\x0Conmouseleave\0\0\0 __widl_f_onmousemove_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bonmousemove\x01\x01\x05self_\x0Bonmousemove\0\0\0$__widl_f_set_onmousemove_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bonmousemove\x01\x02\x05self_\x0Bonmousemove\x0Bonmousemove\0\0\0\x1F__widl_f_onmouseout_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nonmouseout\x01\x01\x05self_\nonmouseout\0\0\0#__widl_f_set_onmouseout_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nonmouseout\x01\x02\x05self_\nonmouseout\nonmouseout\0\0\0 __widl_f_onmouseover_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bonmouseover\x01\x01\x05self_\x0Bonmouseover\0\0\0$__widl_f_set_onmouseover_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bonmouseover\x01\x02\x05self_\x0Bonmouseover\x0Bonmouseover\0\0\0\x1E__widl_f_onmouseup_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonmouseup\x01\x01\x05self_\tonmouseup\0\0\0\"__widl_f_set_onmouseup_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonmouseup\x01\x02\x05self_\tonmouseup\tonmouseup\0\0\0\x1C__widl_f_onwheel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onwheel\x01\x01\x05self_\x07onwheel\0\0\0 __widl_f_set_onwheel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onwheel\x01\x02\x05self_\x07onwheel\x07onwheel\0\0\0\x1C__widl_f_onpause_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onpause\x01\x01\x05self_\x07onpause\0\0\0 __widl_f_set_onpause_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onpause\x01\x02\x05self_\x07onpause\x07onpause\0\0\0\x1B__widl_f_onplay_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06onplay\x01\x01\x05self_\x06onplay\0\0\0\x1F__widl_f_set_onplay_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06onplay\x01\x02\x05self_\x06onplay\x06onplay\0\0\0\x1E__widl_f_onplaying_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonplaying\x01\x01\x05self_\tonplaying\0\0\0\"__widl_f_set_onplaying_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonplaying\x01\x02\x05self_\tonplaying\tonplaying\0\0\0\x1F__widl_f_onprogress_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nonprogress\x01\x01\x05self_\nonprogress\0\0\0#__widl_f_set_onprogress_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nonprogress\x01\x02\x05self_\nonprogress\nonprogress\0\0\0!__widl_f_onratechange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Conratechange\x01\x01\x05self_\x0Conratechange\0\0\0%__widl_f_set_onratechange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Conratechange\x01\x02\x05self_\x0Conratechange\x0Conratechange\0\0\0\x1C__widl_f_onreset_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onreset\x01\x01\x05self_\x07onreset\0\0\0 __widl_f_set_onreset_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onreset\x01\x02\x05self_\x07onreset\x07onreset\0\0\0\x1D__widl_f_onresize_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08onresize\x01\x01\x05self_\x08onresize\0\0\0!__widl_f_set_onresize_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08onresize\x01\x02\x05self_\x08onresize\x08onresize\0\0\0\x1D__widl_f_onscroll_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08onscroll\x01\x01\x05self_\x08onscroll\0\0\0!__widl_f_set_onscroll_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08onscroll\x01\x02\x05self_\x08onscroll\x08onscroll\0\0\0\x1D__widl_f_onseeked_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08onseeked\x01\x01\x05self_\x08onseeked\0\0\0!__widl_f_set_onseeked_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08onseeked\x01\x02\x05self_\x08onseeked\x08onseeked\0\0\0\x1E__widl_f_onseeking_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonseeking\x01\x01\x05self_\tonseeking\0\0\0\"__widl_f_set_onseeking_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonseeking\x01\x02\x05self_\tonseeking\tonseeking\0\0\0\x1D__widl_f_onselect_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08onselect\x01\x01\x05self_\x08onselect\0\0\0!__widl_f_set_onselect_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08onselect\x01\x02\x05self_\x08onselect\x08onselect\0\0\0\x1B__widl_f_onshow_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x06onshow\x01\x01\x05self_\x06onshow\0\0\0\x1F__widl_f_set_onshow_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x06onshow\x01\x02\x05self_\x06onshow\x06onshow\0\0\0\x1E__widl_f_onstalled_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonstalled\x01\x01\x05self_\tonstalled\0\0\0\"__widl_f_set_onstalled_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonstalled\x01\x02\x05self_\tonstalled\tonstalled\0\0\0\x1D__widl_f_onsubmit_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08onsubmit\x01\x01\x05self_\x08onsubmit\0\0\0!__widl_f_set_onsubmit_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08onsubmit\x01\x02\x05self_\x08onsubmit\x08onsubmit\0\0\0\x1E__widl_f_onsuspend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonsuspend\x01\x01\x05self_\tonsuspend\0\0\0\"__widl_f_set_onsuspend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonsuspend\x01\x02\x05self_\tonsuspend\tonsuspend\0\0\0!__widl_f_ontimeupdate_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Contimeupdate\x01\x01\x05self_\x0Contimeupdate\0\0\0%__widl_f_set_ontimeupdate_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Contimeupdate\x01\x02\x05self_\x0Contimeupdate\x0Contimeupdate\0\0\0#__widl_f_onvolumechange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Eonvolumechange\x01\x01\x05self_\x0Eonvolumechange\0\0\0'__widl_f_set_onvolumechange_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Eonvolumechange\x01\x02\x05self_\x0Eonvolumechange\x0Eonvolumechange\0\0\0\x1E__widl_f_onwaiting_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\tonwaiting\x01\x01\x05self_\tonwaiting\0\0\0\"__widl_f_set_onwaiting_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\tonwaiting\x01\x02\x05self_\tonwaiting\tonwaiting\0\0\0\"__widl_f_onselectstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\ronselectstart\x01\x01\x05self_\ronselectstart\0\0\0&__widl_f_set_onselectstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\ronselectstart\x01\x02\x05self_\ronselectstart\ronselectstart\0\0\0\x1D__widl_f_ontoggle_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x08ontoggle\x01\x01\x05self_\x08ontoggle\0\0\0!__widl_f_set_ontoggle_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x08ontoggle\x01\x02\x05self_\x08ontoggle\x08ontoggle\0\0\0$__widl_f_onpointercancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Fonpointercancel\x01\x01\x05self_\x0Fonpointercancel\0\0\0(__widl_f_set_onpointercancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Fonpointercancel\x01\x02\x05self_\x0Fonpointercancel\x0Fonpointercancel\0\0\0\"__widl_f_onpointerdown_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\ronpointerdown\x01\x01\x05self_\ronpointerdown\0\0\0&__widl_f_set_onpointerdown_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\ronpointerdown\x01\x02\x05self_\ronpointerdown\ronpointerdown\0\0\0 __widl_f_onpointerup_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bonpointerup\x01\x01\x05self_\x0Bonpointerup\0\0\0$__widl_f_set_onpointerup_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bonpointerup\x01\x02\x05self_\x0Bonpointerup\x0Bonpointerup\0\0\0\"__widl_f_onpointermove_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\ronpointermove\x01\x01\x05self_\ronpointermove\0\0\0&__widl_f_set_onpointermove_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\ronpointermove\x01\x02\x05self_\ronpointermove\ronpointermove\0\0\0!__widl_f_onpointerout_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Conpointerout\x01\x01\x05self_\x0Conpointerout\0\0\0%__widl_f_set_onpointerout_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Conpointerout\x01\x02\x05self_\x0Conpointerout\x0Conpointerout\0\0\0\"__widl_f_onpointerover_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\ronpointerover\x01\x01\x05self_\ronpointerover\0\0\0&__widl_f_set_onpointerover_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\ronpointerover\x01\x02\x05self_\ronpointerover\ronpointerover\0\0\0#__widl_f_onpointerenter_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Eonpointerenter\x01\x01\x05self_\x0Eonpointerenter\0\0\0'__widl_f_set_onpointerenter_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Eonpointerenter\x01\x02\x05self_\x0Eonpointerenter\x0Eonpointerenter\0\0\0#__widl_f_onpointerleave_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Eonpointerleave\x01\x01\x05self_\x0Eonpointerleave\0\0\0'__widl_f_set_onpointerleave_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Eonpointerleave\x01\x02\x05self_\x0Eonpointerleave\x0Eonpointerleave\0\0\0(__widl_f_ongotpointercapture_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x13ongotpointercapture\x01\x01\x05self_\x13ongotpointercapture\0\0\0,__widl_f_set_ongotpointercapture_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x13ongotpointercapture\x01\x02\x05self_\x13ongotpointercapture\x13ongotpointercapture\0\0\0)__widl_f_onlostpointercapture_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x14onlostpointercapture\x01\x01\x05self_\x14onlostpointercapture\0\0\0-__widl_f_set_onlostpointercapture_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x14onlostpointercapture\x01\x02\x05self_\x14onlostpointercapture\x14onlostpointercapture\0\0\0&__widl_f_onanimationcancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x11onanimationcancel\x01\x01\x05self_\x11onanimationcancel\0\0\0*__widl_f_set_onanimationcancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x11onanimationcancel\x01\x02\x05self_\x11onanimationcancel\x11onanimationcancel\0\0\0#__widl_f_onanimationend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Eonanimationend\x01\x01\x05self_\x0Eonanimationend\0\0\0'__widl_f_set_onanimationend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Eonanimationend\x01\x02\x05self_\x0Eonanimationend\x0Eonanimationend\0\0\0)__widl_f_onanimationiteration_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x14onanimationiteration\x01\x01\x05self_\x14onanimationiteration\0\0\0-__widl_f_set_onanimationiteration_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x14onanimationiteration\x01\x02\x05self_\x14onanimationiteration\x14onanimationiteration\0\0\0%__widl_f_onanimationstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x10onanimationstart\x01\x01\x05self_\x10onanimationstart\0\0\0)__widl_f_set_onanimationstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x10onanimationstart\x01\x02\x05self_\x10onanimationstart\x10onanimationstart\0\0\0'__widl_f_ontransitioncancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x12ontransitioncancel\x01\x01\x05self_\x12ontransitioncancel\0\0\0+__widl_f_set_ontransitioncancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x12ontransitioncancel\x01\x02\x05self_\x12ontransitioncancel\x12ontransitioncancel\0\0\0$__widl_f_ontransitionend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Fontransitionend\x01\x01\x05self_\x0Fontransitionend\0\0\0(__widl_f_set_ontransitionend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Fontransitionend\x01\x02\x05self_\x0Fontransitionend\x0Fontransitionend\0\0\0$__widl_f_ontransitionrun_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Fontransitionrun\x01\x01\x05self_\x0Fontransitionrun\0\0\0(__widl_f_set_ontransitionrun_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Fontransitionrun\x01\x02\x05self_\x0Fontransitionrun\x0Fontransitionrun\0\0\0&__widl_f_ontransitionstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x11ontransitionstart\x01\x01\x05self_\x11ontransitionstart\0\0\0*__widl_f_set_ontransitionstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x11ontransitionstart\x01\x02\x05self_\x11ontransitionstart\x11ontransitionstart\0\0\0)__widl_f_onwebkitanimationend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x14onwebkitanimationend\x01\x01\x05self_\x14onwebkitanimationend\0\0\0-__widl_f_set_onwebkitanimationend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x14onwebkitanimationend\x01\x02\x05self_\x14onwebkitanimationend\x14onwebkitanimationend\0\0\0/__widl_f_onwebkitanimationiteration_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x1Aonwebkitanimationiteration\x01\x01\x05self_\x1Aonwebkitanimationiteration\0\0\03__widl_f_set_onwebkitanimationiteration_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x1Aonwebkitanimationiteration\x01\x02\x05self_\x1Aonwebkitanimationiteration\x1Aonwebkitanimationiteration\0\0\0+__widl_f_onwebkitanimationstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x16onwebkitanimationstart\x01\x01\x05self_\x16onwebkitanimationstart\0\0\0/__widl_f_set_onwebkitanimationstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x16onwebkitanimationstart\x01\x02\x05self_\x16onwebkitanimationstart\x16onwebkitanimationstart\0\0\0*__widl_f_onwebkittransitionend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x15onwebkittransitionend\x01\x01\x05self_\x15onwebkittransitionend\0\0\0.__widl_f_set_onwebkittransitionend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x15onwebkittransitionend\x01\x02\x05self_\x15onwebkittransitionend\x15onwebkittransitionend\0\0\0\x1C__widl_f_onerror_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0 __widl_f_set_onerror_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0!__widl_f_ontouchstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Contouchstart\x01\x01\x05self_\x0Contouchstart\0\0\0%__widl_f_set_ontouchstart_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Contouchstart\x01\x02\x05self_\x0Contouchstart\x0Contouchstart\0\0\0\x1F__widl_f_ontouchend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\nontouchend\x01\x01\x05self_\nontouchend\0\0\0#__widl_f_set_ontouchend_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\nontouchend\x01\x02\x05self_\nontouchend\nontouchend\0\0\0 __widl_f_ontouchmove_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\x0Bontouchmove\x01\x01\x05self_\x0Bontouchmove\0\0\0$__widl_f_set_ontouchmove_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\x0Bontouchmove\x01\x02\x05self_\x0Bontouchmove\x0Bontouchmove\0\0\0\"__widl_f_ontouchcancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x01\rontouchcancel\x01\x01\x05self_\rontouchcancel\0\0\0&__widl_f_set_ontouchcancel_HTMLElement\0\0\0\x01\x0BHTMLElement\x01\0\x02\rontouchcancel\x01\x02\x05self_\rontouchcancel\rontouchcancel\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
