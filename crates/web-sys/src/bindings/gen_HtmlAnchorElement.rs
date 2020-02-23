use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLAnchorElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlAnchorElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlAnchorElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlAnchorElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(65u32);
            inform(110u32);
            inform(99u32);
            inform(104u32);
            inform(111u32);
            inform(114u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlAnchorElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlAnchorElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlAnchorElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlAnchorElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlAnchorElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlAnchorElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlAnchorElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlAnchorElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlAnchorElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlAnchorElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlAnchorElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlAnchorElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlAnchorElement {
            HtmlAnchorElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlAnchorElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlAnchorElement> for HtmlAnchorElement {
        #[inline]
        fn as_ref(&self) -> &HtmlAnchorElement {
            self
        }
    }
    impl From<HtmlAnchorElement> for JsValue {
        #[inline]
        fn from(obj: HtmlAnchorElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlAnchorElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLAnchorElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLAnchorElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLAnchorElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlAnchorElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlAnchorElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlAnchorElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlAnchorElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlAnchorElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAnchorElement> for Element {
    #[inline]
    fn from(obj: HtmlAnchorElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlAnchorElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAnchorElement> for Node {
    #[inline]
    fn from(obj: HtmlAnchorElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlAnchorElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAnchorElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlAnchorElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlAnchorElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlAnchorElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlAnchorElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlAnchorElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/target)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_target(&self, target: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                __widl_f_set_target_HTMLAnchorElement(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_download_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `download` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn download(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_download_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_download_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_download_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_download_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `download` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/download)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_download(&self, download: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_download_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                download: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_download_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            download: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(download);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let download = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(download);
                __widl_f_set_download_HTMLAnchorElement(self_, download)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ping_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `ping` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn ping(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ping_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ping_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ping_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ping_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `ping` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/ping)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_ping(&self, ping: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ping_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ping: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ping_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ping: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ping);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ping = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ping);
                __widl_f_set_ping_HTMLAnchorElement(self_, ping)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn rel(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rel_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rel)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_rel(&self, rel: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rel_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rel_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rel = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rel);
                __widl_f_set_rel_HTMLAnchorElement(self_, rel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_referrer_policy_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_referrer_policy(&self, referrer_policy: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_referrer_policy_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_referrer_policy_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(referrer_policy);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let referrer_policy =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(referrer_policy);
                __widl_f_set_referrer_policy_HTMLAnchorElement(self_, referrer_policy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_list_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "DomTokenList", feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `relList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn rel_list(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_list_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_list_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_list_HTMLAnchorElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hreflang_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `hreflang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn hreflang(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hreflang_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hreflang_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hreflang_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hreflang_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `hreflang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hreflang)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_hreflang(&self, hreflang: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hreflang_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hreflang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hreflang_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hreflang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hreflang);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hreflang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hreflang);
                __widl_f_set_hreflang_HTMLAnchorElement(self_, hreflang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/type)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLAnchorElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_HTMLAnchorElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/text)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_HTMLAnchorElement(self_, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_coords_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `coords` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn coords(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_coords_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_coords_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_coords_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_coords_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `coords` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/coords)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_coords(&self, coords: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_coords_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                coords: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_coords_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            coords: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(coords);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let coords = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(coords);
                __widl_f_set_coords_HTMLAnchorElement(self_, coords)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_charset_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `charset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn charset(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_charset_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_charset_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_charset_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_charset_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `charset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/charset)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_charset(&self, charset: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_charset_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_charset_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            charset: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(charset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let charset = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(charset);
                __widl_f_set_charset_HTMLAnchorElement(self_, charset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/name)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_HTMLAnchorElement(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rev_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `rev` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn rev(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rev_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rev_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rev_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rev_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `rev` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/rev)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_rev(&self, rev: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rev_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rev: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rev_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rev: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rev);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rev = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rev);
                __widl_f_set_rev_HTMLAnchorElement(self_, rev)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_shape_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `shape` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn shape(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_shape_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_shape_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_shape_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_shape_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `shape` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/shape)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_shape(&self, shape: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_shape_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                shape: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_shape_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            shape: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(shape);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let shape = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(shape);
                __widl_f_set_shape_HTMLAnchorElement(self_, shape)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_href_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/href)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_href(&self, href: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_href_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_href_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            href: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(href);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let href = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(href);
                __widl_f_set_href_HTMLAnchorElement(self_, href)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_origin_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `origin` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/origin)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn origin(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_origin_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_origin_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_origin_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_protocol_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn protocol(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_protocol_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_protocol_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_protocol_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_protocol_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `protocol` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/protocol)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_protocol(&self, protocol: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_protocol_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_protocol_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            protocol: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(protocol);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let protocol = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(protocol);
                __widl_f_set_protocol_HTMLAnchorElement(self_, protocol)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_username_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `username` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn username(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_username_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_username_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_username_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_username_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `username` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/username)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_username(&self, username: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_username_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                username: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_username_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            username: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(username);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let username = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(username);
                __widl_f_set_username_HTMLAnchorElement(self_, username)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_password_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `password` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn password(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_password_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_password_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_password_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_password_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `password` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/password)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_password(&self, password: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_password_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                password: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_password_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            password: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(password);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let password = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(password);
                __widl_f_set_password_HTMLAnchorElement(self_, password)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_host_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `host` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn host(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_host_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_host_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_host_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_host_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `host` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/host)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_host(&self, host: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_host_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_host_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            host: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(host);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let host = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(host);
                __widl_f_set_host_HTMLAnchorElement(self_, host)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hostname_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn hostname(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hostname_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hostname_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hostname_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hostname_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `hostname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hostname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_hostname(&self, hostname: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hostname_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hostname_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hostname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hostname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hostname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hostname);
                __widl_f_set_hostname_HTMLAnchorElement(self_, hostname)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_port_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `port` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn port(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_port_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_port_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_port_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_port_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `port` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/port)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_port(&self, port: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_port_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_port_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            port: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(port);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let port = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(port);
                __widl_f_set_port_HTMLAnchorElement(self_, port)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pathname_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn pathname(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pathname_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pathname_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pathname_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pathname_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `pathname` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/pathname)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_pathname(&self, pathname: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pathname_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pathname_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pathname: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pathname);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pathname = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pathname);
                __widl_f_set_pathname_HTMLAnchorElement(self_, pathname)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_search_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `search` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn search(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_search_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_search_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_search_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_search_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `search` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/search)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_search(&self, search: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_search_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_search_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            search: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(search);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let search = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(search);
                __widl_f_set_search_HTMLAnchorElement(self_, search)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hash_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `hash` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn hash(&self) -> String {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hash_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hash_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hash_HTMLAnchorElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlAnchorElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hash_HTMLAnchorElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlAnchorElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlAnchorElement {
    #[cfg(all(feature = "HtmlAnchorElement",))]
    #[allow(bad_style)]
    #[doc = "The `hash` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement/hash)\n\n*This API requires the following crate features to be activated: `HtmlAnchorElement`*"]
    #[allow(clippy::all)]
    pub fn set_hash(&self, hash: &str) {
        #[cfg(all(feature = "HtmlAnchorElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hash_HTMLAnchorElement(
                self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hash_HTMLAnchorElement(
            self_: <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            hash: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(hash);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlAnchorElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hash = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hash);
                __widl_f_set_hash_HTMLAnchorElement(self_, hash)
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
pub static __WASM_BINDGEN_GENERATED_34e197708b27136e: [u8; 4456usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}&\x11\0\0\0\01\0\0\x02\x11HTMLAnchorElement#__widl_instanceof_HTMLAnchorElement\0\0\0\0!__widl_f_target_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0%__widl_f_set_target_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x06target\x01\x02\x05self_\x06target\x06target\0\0\0#__widl_f_download_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08download\x01\x01\x05self_\x08download\0\0\0'__widl_f_set_download_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08download\x01\x02\x05self_\x08download\x08download\0\0\0\x1F__widl_f_ping_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04ping\x01\x01\x05self_\x04ping\0\0\0#__widl_f_set_ping_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04ping\x01\x02\x05self_\x04ping\x04ping\0\0\0\x1E__widl_f_rel_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x03rel\x01\x01\x05self_\x03rel\0\0\0\"__widl_f_set_rel_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x03rel\x01\x02\x05self_\x03rel\x03rel\0\0\0*__widl_f_referrer_policy_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0.__widl_f_set_referrer_policy_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x0EreferrerPolicy\x01\x02\x05self_\x0Freferrer_policy\x0EreferrerPolicy\0\0\0#__widl_f_rel_list_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x07relList\x01\x01\x05self_\x07relList\0\0\0#__widl_f_hreflang_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08hreflang\x01\x01\x05self_\x08hreflang\0\0\0'__widl_f_set_hreflang_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08hreflang\x01\x02\x05self_\x08hreflang\x08hreflang\0\0\0\x1F__widl_f_type_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0#__widl_f_set_type_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0\x1F__widl_f_text_HTMLAnchorElement\x01\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0#__widl_f_set_text_HTMLAnchorElement\x01\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0!__widl_f_coords_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x06coords\x01\x01\x05self_\x06coords\0\0\0%__widl_f_set_coords_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x06coords\x01\x02\x05self_\x06coords\x06coords\0\0\0\"__widl_f_charset_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x07charset\x01\x01\x05self_\x07charset\0\0\0&__widl_f_set_charset_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x07charset\x01\x02\x05self_\x07charset\x07charset\0\0\0\x1F__widl_f_name_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0#__widl_f_set_name_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0\x1E__widl_f_rev_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x03rev\x01\x01\x05self_\x03rev\0\0\0\"__widl_f_set_rev_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x03rev\x01\x02\x05self_\x03rev\x03rev\0\0\0 __widl_f_shape_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x05shape\x01\x01\x05self_\x05shape\0\0\0$__widl_f_set_shape_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x05shape\x01\x02\x05self_\x05shape\x05shape\0\0\0\x1F__widl_f_href_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0#__widl_f_set_href_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04href\x01\x02\x05self_\x04href\x04href\0\0\0!__widl_f_origin_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x06origin\x01\x01\x05self_\x06origin\0\0\0#__widl_f_protocol_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08protocol\x01\x01\x05self_\x08protocol\0\0\0'__widl_f_set_protocol_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08protocol\x01\x02\x05self_\x08protocol\x08protocol\0\0\0#__widl_f_username_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08username\x01\x01\x05self_\x08username\0\0\0'__widl_f_set_username_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08username\x01\x02\x05self_\x08username\x08username\0\0\0#__widl_f_password_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08password\x01\x01\x05self_\x08password\0\0\0'__widl_f_set_password_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08password\x01\x02\x05self_\x08password\x08password\0\0\0\x1F__widl_f_host_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04host\x01\x01\x05self_\x04host\0\0\0#__widl_f_set_host_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04host\x01\x02\x05self_\x04host\x04host\0\0\0#__widl_f_hostname_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08hostname\x01\x01\x05self_\x08hostname\0\0\0'__widl_f_set_hostname_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08hostname\x01\x02\x05self_\x08hostname\x08hostname\0\0\0\x1F__widl_f_port_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04port\x01\x01\x05self_\x04port\0\0\0#__widl_f_set_port_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04port\x01\x02\x05self_\x04port\x04port\0\0\0#__widl_f_pathname_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x08pathname\x01\x01\x05self_\x08pathname\0\0\0'__widl_f_set_pathname_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x08pathname\x01\x02\x05self_\x08pathname\x08pathname\0\0\0!__widl_f_search_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x06search\x01\x01\x05self_\x06search\0\0\0%__widl_f_set_search_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x06search\x01\x02\x05self_\x06search\x06search\0\0\0\x1F__widl_f_hash_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x01\x04hash\x01\x01\x05self_\x04hash\0\0\0#__widl_f_set_hash_HTMLAnchorElement\0\0\0\x01\x11HTMLAnchorElement\x01\0\x02\x04hash\x01\x02\x05self_\x04hash\x04hash\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
