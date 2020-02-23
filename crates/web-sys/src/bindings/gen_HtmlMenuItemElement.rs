use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLMenuItemElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlMenuItemElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlMenuItemElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlMenuItemElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(77u32);
            inform(101u32);
            inform(110u32);
            inform(117u32);
            inform(73u32);
            inform(116u32);
            inform(101u32);
            inform(109u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlMenuItemElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlMenuItemElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlMenuItemElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlMenuItemElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlMenuItemElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlMenuItemElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlMenuItemElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlMenuItemElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlMenuItemElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlMenuItemElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlMenuItemElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlMenuItemElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlMenuItemElement {
            HtmlMenuItemElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlMenuItemElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlMenuItemElement> for HtmlMenuItemElement {
        #[inline]
        fn as_ref(&self) -> &HtmlMenuItemElement {
            self
        }
    }
    impl From<HtmlMenuItemElement> for JsValue {
        #[inline]
        fn from(obj: HtmlMenuItemElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlMenuItemElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLMenuItemElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLMenuItemElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLMenuItemElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlMenuItemElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlMenuItemElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlMenuItemElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlMenuItemElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlMenuItemElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMenuItemElement> for Element {
    #[inline]
    fn from(obj: HtmlMenuItemElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlMenuItemElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMenuItemElement> for Node {
    #[inline]
    fn from(obj: HtmlMenuItemElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlMenuItemElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMenuItemElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlMenuItemElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlMenuItemElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlMenuItemElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlMenuItemElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlMenuItemElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> String {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_HTMLMenuItemElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/type)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: &str) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_HTMLMenuItemElement(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_HTMLMenuItemElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_label_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `label` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/label)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_label(&self, label: &str) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_label_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                label: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_label_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let label = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(label);
                __widl_f_set_label_HTMLMenuItemElement(self_, label)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_icon_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `icon` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn icon(&self) -> String {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_icon_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_icon_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_icon_HTMLMenuItemElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_icon_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `icon` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/icon)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_icon(&self, icon: &str) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_icon_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                icon: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_icon_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            icon: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(icon);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let icon = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(icon);
                __widl_f_set_icon_HTMLMenuItemElement(self_, icon)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_disabled_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn disabled(&self) -> bool {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_disabled_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_disabled_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_disabled_HTMLMenuItemElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_disabled_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `disabled` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/disabled)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_disabled(&self, disabled: bool) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_disabled_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                disabled: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_disabled_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let disabled = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(disabled);
                __widl_f_set_disabled_HTMLMenuItemElement(self_, disabled)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_checked_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `checked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn checked(&self) -> bool {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_checked_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_checked_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_checked_HTMLMenuItemElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_checked_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `checked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/checked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_checked(&self, checked: bool) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_checked_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_checked_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(checked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let checked = <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(checked);
                __widl_f_set_checked_HTMLMenuItemElement(self_, checked)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_radiogroup_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `radiogroup` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn radiogroup(&self) -> String {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_radiogroup_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_radiogroup_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_radiogroup_HTMLMenuItemElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_radiogroup_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `radiogroup` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/radiogroup)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_radiogroup(&self, radiogroup: &str) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_radiogroup_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                radiogroup: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_radiogroup_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            radiogroup: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(radiogroup);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let radiogroup = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(radiogroup);
                __widl_f_set_radiogroup_HTMLMenuItemElement(self_, radiogroup)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_default_checked_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultChecked` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn default_checked(&self) -> bool {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_default_checked_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_default_checked_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_default_checked_HTMLMenuItemElement(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlMenuItemElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_default_checked_HTMLMenuItemElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlMenuItemElement as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlMenuItemElement {
    #[cfg(all(feature = "HtmlMenuItemElement",))]
    #[allow(bad_style)]
    #[doc = "The `defaultChecked` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMenuItemElement/defaultChecked)\n\n*This API requires the following crate features to be activated: `HtmlMenuItemElement`*"]
    #[allow(clippy::all)]
    pub fn set_default_checked(&self, default_checked: bool) {
        #[cfg(all(feature = "HtmlMenuItemElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_default_checked_HTMLMenuItemElement(
                self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                default_checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_default_checked_HTMLMenuItemElement(
            self_: <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            default_checked: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(default_checked);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlMenuItemElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let default_checked =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(default_checked);
                __widl_f_set_default_checked_HTMLMenuItemElement(self_, default_checked)
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
pub static __WASM_BINDGEN_GENERATED_0835b46bd904d3ff: [u8; 1550usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xCC\x05\0\0\0\0\x0F\0\0\x02\x13HTMLMenuItemElement%__widl_instanceof_HTMLMenuItemElement\0\0\0\0!__widl_f_type_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0%__widl_f_set_type_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0\"__widl_f_label_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0&__widl_f_set_label_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\x05label\x01\x02\x05self_\x05label\x05label\0\0\0!__widl_f_icon_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\x04icon\x01\x01\x05self_\x04icon\0\0\0%__widl_f_set_icon_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\x04icon\x01\x02\x05self_\x04icon\x04icon\0\0\0%__widl_f_disabled_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\x08disabled\x01\x01\x05self_\x08disabled\0\0\0)__widl_f_set_disabled_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\x08disabled\x01\x02\x05self_\x08disabled\x08disabled\0\0\0$__widl_f_checked_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\x07checked\x01\x01\x05self_\x07checked\0\0\0(__widl_f_set_checked_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\x07checked\x01\x02\x05self_\x07checked\x07checked\0\0\0'__widl_f_radiogroup_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\nradiogroup\x01\x01\x05self_\nradiogroup\0\0\0+__widl_f_set_radiogroup_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\nradiogroup\x01\x02\x05self_\nradiogroup\nradiogroup\0\0\0,__widl_f_default_checked_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x01\x0EdefaultChecked\x01\x01\x05self_\x0EdefaultChecked\0\0\00__widl_f_set_default_checked_HTMLMenuItemElement\0\0\0\x01\x13HTMLMenuItemElement\x01\0\x02\x0EdefaultChecked\x01\x02\x05self_\x0Fdefault_checked\x0EdefaultChecked\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
