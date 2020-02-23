use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLHRElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlHrElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlHrElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlHrElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(72u32);
            inform(82u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlHrElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlHrElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlHrElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlHrElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlHrElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlHrElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlHrElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlHrElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlHrElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlHrElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlHrElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlHrElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlHrElement {
            HtmlHrElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlHrElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlHrElement> for HtmlHrElement {
        #[inline]
        fn as_ref(&self) -> &HtmlHrElement {
            self
        }
    }
    impl From<HtmlHrElement> for JsValue {
        #[inline]
        fn from(obj: HtmlHrElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlHrElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLHRElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLHRElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLHRElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlHrElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlHrElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlHrElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlHrElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlHrElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHrElement> for Element {
    #[inline]
    fn from(obj: HtmlHrElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlHrElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHrElement> for Node {
    #[inline]
    fn from(obj: HtmlHrElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlHrElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHrElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlHrElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlHrElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlHrElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlHrElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlHrElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_align_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn align(&self) -> String {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_align_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_align_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_align_HTMLHRElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_align_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `align` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/align)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn set_align(&self, align: &str) {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_align_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                align: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_align_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let align = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(align);
                __widl_f_set_align_HTMLHRElement(self_, align)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_color_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `color` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn color(&self) -> String {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_color_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_color_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_color_HTMLHRElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_color_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `color` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/color)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn set_color(&self, color: &str) {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_color_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_color_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            color: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(color);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let color = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(color);
                __widl_f_set_color_HTMLHRElement(self_, color)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_no_shade_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `noShade` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn no_shade(&self) -> bool {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_no_shade_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_no_shade_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_no_shade_HTMLHRElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_no_shade_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `noShade` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/noShade)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn set_no_shade(&self, no_shade: bool) {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_no_shade_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                no_shade: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_no_shade_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            no_shade: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(no_shade);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let no_shade = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(no_shade);
                __widl_f_set_no_shade_HTMLHRElement(self_, no_shade)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_size_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn size(&self) -> String {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_size_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_size_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_size_HTMLHRElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_size_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `size` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/size)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn set_size(&self, size: &str) {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_size_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_size_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_set_size_HTMLHRElement(self_, size)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> String {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLHRElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlHrElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLHRElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlHrElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlHrElement {
    #[cfg(all(feature = "HtmlHrElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHRElement/width)\n\n*This API requires the following crate features to be activated: `HtmlHrElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: &str) {
        #[cfg(all(feature = "HtmlHrElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLHRElement(
                self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLHRElement(
            self_: <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&HtmlHrElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLHRElement(self_, width)
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
pub static __WASM_BINDGEN_GENERATED_1e04dc09ddaa0f37: [u8; 945usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}o\x03\0\0\0\0\x0B\0\0\x02\rHTMLHRElement\x1F__widl_instanceof_HTMLHRElement\0\0\0\0\x1C__widl_f_align_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x01\x05align\x01\x01\x05self_\x05align\0\0\0 __widl_f_set_align_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x02\x05align\x01\x02\x05self_\x05align\x05align\0\0\0\x1C__widl_f_color_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x01\x05color\x01\x01\x05self_\x05color\0\0\0 __widl_f_set_color_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x02\x05color\x01\x02\x05self_\x05color\x05color\0\0\0\x1F__widl_f_no_shade_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x01\x07noShade\x01\x01\x05self_\x07noShade\0\0\0#__widl_f_set_no_shade_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x02\x07noShade\x01\x02\x05self_\x08no_shade\x07noShade\0\0\0\x1B__widl_f_size_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x01\x04size\x01\x01\x05self_\x04size\0\0\0\x1F__widl_f_set_size_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x02\x04size\x01\x02\x05self_\x04size\x04size\0\0\0\x1C__widl_f_width_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0 __widl_f_set_width_HTMLHRElement\0\0\0\x01\rHTMLHRElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
