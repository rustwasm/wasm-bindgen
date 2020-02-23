use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLMeterElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlMeterElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlMeterElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlMeterElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(77u32);
            inform(101u32);
            inform(116u32);
            inform(101u32);
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
    impl core::ops::Deref for HtmlMeterElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlMeterElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlMeterElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlMeterElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlMeterElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlMeterElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlMeterElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlMeterElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlMeterElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlMeterElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlMeterElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlMeterElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlMeterElement {
            HtmlMeterElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlMeterElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlMeterElement> for HtmlMeterElement {
        #[inline]
        fn as_ref(&self) -> &HtmlMeterElement {
            self
        }
    }
    impl From<HtmlMeterElement> for JsValue {
        #[inline]
        fn from(obj: HtmlMeterElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlMeterElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLMeterElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLMeterElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLMeterElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlMeterElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlMeterElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlMeterElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlMeterElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlMeterElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMeterElement> for Element {
    #[inline]
    fn from(obj: HtmlMeterElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlMeterElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMeterElement> for Node {
    #[inline]
    fn from(obj: HtmlMeterElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlMeterElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMeterElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlMeterElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlMeterElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMeterElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlMeterElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlMeterElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_value_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn value(&self) -> f64 {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_value_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_value_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_value_HTMLMeterElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_value_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `value` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/value)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn set_value(&self, value: f64) {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_value_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_value_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_set_value_HTMLMeterElement(self_, value)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_min_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `min` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn min(&self) -> f64 {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_min_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_min_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_min_HTMLMeterElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_min_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `min` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/min)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn set_min(&self, min: f64) {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_min_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                min: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_min_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            min: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let min = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(min);
                __widl_f_set_min_HTMLMeterElement(self_, min)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `max` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn max(&self) -> f64 {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_HTMLMeterElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `max` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/max)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn set_max(&self, max: f64) {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max);
                __widl_f_set_max_HTMLMeterElement(self_, max)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_low_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `low` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn low(&self) -> f64 {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_low_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_low_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_low_HTMLMeterElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_low_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `low` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/low)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn set_low(&self, low: f64) {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_low_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                low: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_low_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            low: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(low);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let low = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(low);
                __widl_f_set_low_HTMLMeterElement(self_, low)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_high_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `high` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn high(&self) -> f64 {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_high_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_high_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_high_HTMLMeterElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_high_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `high` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/high)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn set_high(&self, high: f64) {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_high_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                high: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_high_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            high: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(high);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let high = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(high);
                __widl_f_set_high_HTMLMeterElement(self_, high)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_optimum_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `optimum` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn optimum(&self) -> f64 {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_optimum_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_optimum_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_optimum_HTMLMeterElement(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_optimum_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement",))]
    #[allow(bad_style)]
    #[doc = "The `optimum` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/optimum)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`*"]
    #[allow(clippy::all)]
    pub fn set_optimum(&self, optimum: f64) {
        #[cfg(all(feature = "HtmlMeterElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_optimum_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                optimum: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_optimum_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            optimum: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(optimum);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let optimum = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(optimum);
                __widl_f_set_optimum_HTMLMeterElement(self_, optimum)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMeterElement", feature = "NodeList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_labels_HTMLMeterElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMeterElement as WasmDescribe>::describe();
    <NodeList as WasmDescribe>::describe();
}
impl HtmlMeterElement {
    #[cfg(all(feature = "HtmlMeterElement", feature = "NodeList",))]
    #[allow(bad_style)]
    #[doc = "The `labels` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMeterElement/labels)\n\n*This API requires the following crate features to be activated: `HtmlMeterElement`, `NodeList`*"]
    #[allow(clippy::all)]
    pub fn labels(&self) -> NodeList {
        #[cfg(all(feature = "HtmlMeterElement", feature = "NodeList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_labels_HTMLMeterElement(
                self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <NodeList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_labels_HTMLMeterElement(
            self_: <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMeterElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_labels_HTMLMeterElement(self_)
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
pub static __WASM_BINDGEN_GENERATED_e56a90dd64387038: [u8; 1216usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}~\x04\0\0\0\0\x0E\0\0\x02\x10HTMLMeterElement\"__widl_instanceof_HTMLMeterElement\0\0\0\0\x1F__widl_f_value_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x05value\x01\x01\x05self_\x05value\0\0\0#__widl_f_set_value_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x02\x05value\x01\x02\x05self_\x05value\x05value\0\0\0\x1D__widl_f_min_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x03min\x01\x01\x05self_\x03min\0\0\0!__widl_f_set_min_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x02\x03min\x01\x02\x05self_\x03min\x03min\0\0\0\x1D__widl_f_max_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x03max\x01\x01\x05self_\x03max\0\0\0!__widl_f_set_max_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x02\x03max\x01\x02\x05self_\x03max\x03max\0\0\0\x1D__widl_f_low_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x03low\x01\x01\x05self_\x03low\0\0\0!__widl_f_set_low_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x02\x03low\x01\x02\x05self_\x03low\x03low\0\0\0\x1E__widl_f_high_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x04high\x01\x01\x05self_\x04high\0\0\0\"__widl_f_set_high_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x02\x04high\x01\x02\x05self_\x04high\x04high\0\0\0!__widl_f_optimum_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x07optimum\x01\x01\x05self_\x07optimum\0\0\0%__widl_f_set_optimum_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x02\x07optimum\x01\x02\x05self_\x07optimum\x07optimum\0\0\0 __widl_f_labels_HTMLMeterElement\0\0\0\x01\x10HTMLMeterElement\x01\0\x01\x06labels\x01\x01\x05self_\x06labels\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
