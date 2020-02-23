use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgaElement {
    obj: SvgGraphicsElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgaElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgaElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(65u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgaElement {
        type Target = SvgGraphicsElement;
        #[inline]
        fn deref(&self) -> &SvgGraphicsElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgaElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgaElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgaElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgaElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgaElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgaElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgaElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgaElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgaElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgaElement {
        #[inline]
        fn from(obj: JsValue) -> SvgaElement {
            SvgaElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgaElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgaElement> for SvgaElement {
        #[inline]
        fn as_ref(&self) -> &SvgaElement {
            self
        }
    }
    impl From<SvgaElement> for JsValue {
        #[inline]
        fn from(obj: SvgaElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgaElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgaElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgaElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgaElement> for SvgGraphicsElement {
    #[inline]
    fn from(obj: SvgaElement) -> SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgGraphicsElement> for SvgaElement {
    #[inline]
    fn as_ref(&self) -> &SvgGraphicsElement {
        use wasm_bindgen::JsCast;
        SvgGraphicsElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgaElement> for SvgElement {
    #[inline]
    fn from(obj: SvgaElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgaElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgaElement> for Element {
    #[inline]
    fn from(obj: SvgaElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgaElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgaElement> for Node {
    #[inline]
    fn from(obj: SvgaElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgaElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgaElement> for EventTarget {
    #[inline]
    fn from(obj: SvgaElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgaElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgaElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgaElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgaElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/target)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_SVGAElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_download_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `download` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn download(&self) -> String {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_download_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_download_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_download_SVGAElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_download_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `download` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/download)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_download(&self, download: &str) {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_download_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                download: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_download_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let download = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(download);
                __widl_f_set_download_SVGAElement(self_, download)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ping_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `ping` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn ping(&self) -> String {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ping_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ping_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ping_SVGAElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ping_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `ping` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/ping)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_ping(&self, ping: &str) {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ping_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ping: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ping_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ping = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ping);
                __widl_f_set_ping_SVGAElement(self_, ping)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn rel(&self) -> String {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_SVGAElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rel_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `rel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/rel)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_rel(&self, rel: &str) {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rel_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rel: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rel_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rel = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rel);
                __widl_f_set_rel_SVGAElement(self_, rel)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_referrer_policy_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn referrer_policy(&self) -> String {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_referrer_policy_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_referrer_policy_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_referrer_policy_SVGAElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_referrer_policy_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `referrerPolicy` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/referrerPolicy)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_referrer_policy(&self, referrer_policy: &str) {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_referrer_policy_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                referrer_policy: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_referrer_policy_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let referrer_policy =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(referrer_policy);
                __widl_f_set_referrer_policy_SVGAElement(self_, referrer_policy)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomTokenList", feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rel_list_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <DomTokenList as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "DomTokenList", feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `relList` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/relList)\n\n*This API requires the following crate features to be activated: `DomTokenList`, `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn rel_list(&self) -> DomTokenList {
        #[cfg(all(feature = "DomTokenList", feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rel_list_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rel_list_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rel_list_SVGAElement(self_)
            };
            <DomTokenList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_hreflang_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hreflang` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn hreflang(&self) -> String {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_hreflang_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_hreflang_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_hreflang_SVGAElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_hreflang_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `hreflang` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/hreflang)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_hreflang(&self, hreflang: &str) {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_hreflang_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                hreflang: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_hreflang_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let hreflang = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(hreflang);
                __widl_f_set_hreflang_SVGAElement(self_, hreflang)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_SVGAElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/type)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_SVGAElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_text_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn text(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_text_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_text_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_text_SVGAElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_text_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgaElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `text` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/text)\n\n*This API requires the following crate features to be activated: `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn set_text(&self, text: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_text_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                text: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_text_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let text = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(text);
                __widl_f_set_text_SVGAElement(self_, text)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAnimatedString", feature = "SvgaElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_href_SVGAElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgaElement as WasmDescribe>::describe();
    <SvgAnimatedString as WasmDescribe>::describe();
}
impl SvgaElement {
    #[cfg(all(feature = "SvgAnimatedString", feature = "SvgaElement",))]
    #[allow(bad_style)]
    #[doc = "The `href` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAElement/href)\n\n*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgaElement`*"]
    #[allow(clippy::all)]
    pub fn href(&self) -> SvgAnimatedString {
        #[cfg(all(feature = "SvgAnimatedString", feature = "SvgaElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_href_SVGAElement(
                self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_href_SVGAElement(
            self_: <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SvgaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_href_SVGAElement(self_)
            };
            <SvgAnimatedString as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_7a2232d705e1188e: [u8; 1475usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x81\x05\0\0\0\0\x12\0\0\x02\x0BSVGAElement\x1D__widl_instanceof_SVGAElement\0\0\0\0\x1B__widl_f_target_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0\x1D__widl_f_download_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x08download\x01\x01\x05self_\x08download\0\0\0!__widl_f_set_download_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x02\x08download\x01\x02\x05self_\x08download\x08download\0\0\0\x19__widl_f_ping_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x04ping\x01\x01\x05self_\x04ping\0\0\0\x1D__widl_f_set_ping_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x02\x04ping\x01\x02\x05self_\x04ping\x04ping\0\0\0\x18__widl_f_rel_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x03rel\x01\x01\x05self_\x03rel\0\0\0\x1C__widl_f_set_rel_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x02\x03rel\x01\x02\x05self_\x03rel\x03rel\0\0\0$__widl_f_referrer_policy_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x0EreferrerPolicy\x01\x01\x05self_\x0EreferrerPolicy\0\0\0(__widl_f_set_referrer_policy_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x02\x0EreferrerPolicy\x01\x02\x05self_\x0Freferrer_policy\x0EreferrerPolicy\0\0\0\x1D__widl_f_rel_list_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x07relList\x01\x01\x05self_\x07relList\0\0\0\x1D__widl_f_hreflang_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x08hreflang\x01\x01\x05self_\x08hreflang\0\0\0!__widl_f_set_hreflang_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x02\x08hreflang\x01\x02\x05self_\x08hreflang\x08hreflang\0\0\0\x19__widl_f_type_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\x1D__widl_f_set_type_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0\x19__widl_f_text_SVGAElement\x01\0\0\x01\x0BSVGAElement\x01\0\x01\x04text\x01\x01\x05self_\x04text\0\0\0\x1D__widl_f_set_text_SVGAElement\x01\0\0\x01\x0BSVGAElement\x01\0\x02\x04text\x01\x02\x05self_\x04text\x04text\0\0\0\x19__widl_f_href_SVGAElement\0\0\0\x01\x0BSVGAElement\x01\0\x01\x04href\x01\x01\x05self_\x04href\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
