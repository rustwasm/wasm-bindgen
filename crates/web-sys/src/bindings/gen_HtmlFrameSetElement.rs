use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLFrameSetElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlFrameSetElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlFrameSetElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlFrameSetElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(70u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(83u32);
            inform(101u32);
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
    impl core::ops::Deref for HtmlFrameSetElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlFrameSetElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlFrameSetElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlFrameSetElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlFrameSetElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlFrameSetElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlFrameSetElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlFrameSetElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlFrameSetElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlFrameSetElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlFrameSetElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlFrameSetElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlFrameSetElement {
            HtmlFrameSetElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlFrameSetElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlFrameSetElement> for HtmlFrameSetElement {
        #[inline]
        fn as_ref(&self) -> &HtmlFrameSetElement {
            self
        }
    }
    impl From<HtmlFrameSetElement> for JsValue {
        #[inline]
        fn from(obj: HtmlFrameSetElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlFrameSetElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLFrameSetElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLFrameSetElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLFrameSetElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlFrameSetElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlFrameSetElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlFrameSetElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlFrameSetElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlFrameSetElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameSetElement> for Element {
    #[inline]
    fn from(obj: HtmlFrameSetElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlFrameSetElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameSetElement> for Node {
    #[inline]
    fn from(obj: HtmlFrameSetElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlFrameSetElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameSetElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlFrameSetElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlFrameSetElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlFrameSetElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlFrameSetElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlFrameSetElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cols_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `cols` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn cols(&self) -> String {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cols_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cols_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cols_HTMLFrameSetElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cols_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `cols` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/cols)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_cols(&self, cols: &str) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cols_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cols_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cols);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cols = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cols);
                __widl_f_set_cols_HTMLFrameSetElement(self_, cols)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rows_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `rows` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn rows(&self) -> String {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rows_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rows_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rows_HTMLFrameSetElement(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rows_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `rows` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/rows)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_rows(&self, rows: &str) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rows_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rows: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rows_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rows: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rows);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rows = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rows);
                __widl_f_set_rows_HTMLFrameSetElement(self_, rows)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onafterprint_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onafterprint` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onafterprint(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onafterprint_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onafterprint_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onafterprint_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onafterprint_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onafterprint` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onafterprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onafterprint(&self, onafterprint: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onafterprint_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onafterprint : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onafterprint_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onafterprint: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onafterprint);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onafterprint =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onafterprint,
                    );
                __widl_f_set_onafterprint_HTMLFrameSetElement(self_, onafterprint)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onbeforeprint_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeprint` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onbeforeprint(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onbeforeprint_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onbeforeprint_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onbeforeprint_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onbeforeprint_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeprint` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeprint)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onbeforeprint(&self, onbeforeprint: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onbeforeprint_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onbeforeprint : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onbeforeprint_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onbeforeprint: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onbeforeprint);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onbeforeprint =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onbeforeprint,
                    );
                __widl_f_set_onbeforeprint_HTMLFrameSetElement(self_, onbeforeprint)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onbeforeunload_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeunload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onbeforeunload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onbeforeunload_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onbeforeunload_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onbeforeunload_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onbeforeunload_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onbeforeunload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onbeforeunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onbeforeunload(&self, onbeforeunload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onbeforeunload_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onbeforeunload : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onbeforeunload_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onbeforeunload : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onbeforeunload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onbeforeunload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onbeforeunload,
                    );
                __widl_f_set_onbeforeunload_HTMLFrameSetElement(self_, onbeforeunload)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onhashchange_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onhashchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onhashchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onhashchange_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onhashchange_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onhashchange_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onhashchange_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onhashchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onhashchange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onhashchange(&self, onhashchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onhashchange_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onhashchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onhashchange_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onhashchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onhashchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onhashchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onhashchange,
                    );
                __widl_f_set_onhashchange_HTMLFrameSetElement(self_, onhashchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onlanguagechange_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlanguagechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onlanguagechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onlanguagechange_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onlanguagechange_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onlanguagechange_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onlanguagechange_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onlanguagechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onlanguagechange)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onlanguagechange(&self, onlanguagechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onlanguagechange_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onlanguagechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onlanguagechange_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onlanguagechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onlanguagechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onlanguagechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onlanguagechange,
                    );
                __widl_f_set_onlanguagechange_HTMLFrameSetElement(self_, onlanguagechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onmessage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_HTMLFrameSetElement(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessageerror_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onmessageerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessageerror_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessageerror_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessageerror_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessageerror_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onmessageerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onmessageerror)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onmessageerror(&self, onmessageerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessageerror_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessageerror_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onmessageerror : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onmessageerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessageerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessageerror,
                    );
                __widl_f_set_onmessageerror_HTMLFrameSetElement(self_, onmessageerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onoffline_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onoffline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onoffline(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onoffline_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onoffline_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onoffline_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onoffline_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onoffline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onoffline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onoffline(&self, onoffline: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onoffline_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onoffline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onoffline_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onoffline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onoffline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onoffline =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onoffline,
                    );
                __widl_f_set_onoffline_HTMLFrameSetElement(self_, onoffline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ononline_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `ononline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn ononline(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ononline_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ononline_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ononline_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ononline_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `ononline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/ononline)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_ononline(&self, ononline: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ononline_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ononline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ononline_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ononline: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ononline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ononline =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ononline,
                    );
                __widl_f_set_ononline_HTMLFrameSetElement(self_, ononline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpagehide_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpagehide` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onpagehide(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpagehide_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpagehide_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpagehide_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpagehide_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpagehide` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpagehide)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpagehide(&self, onpagehide: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpagehide_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpagehide : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpagehide_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpagehide: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpagehide);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpagehide =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpagehide,
                    );
                __widl_f_set_onpagehide_HTMLFrameSetElement(self_, onpagehide)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpageshow_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpageshow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onpageshow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpageshow_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpageshow_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpageshow_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpageshow_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpageshow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpageshow)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpageshow(&self, onpageshow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpageshow_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpageshow : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpageshow_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpageshow: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpageshow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpageshow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpageshow,
                    );
                __widl_f_set_onpageshow_HTMLFrameSetElement(self_, onpageshow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onpopstate_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpopstate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onpopstate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onpopstate_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onpopstate_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onpopstate_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onpopstate_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onpopstate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onpopstate)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onpopstate(&self, onpopstate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onpopstate_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onpopstate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onpopstate_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onpopstate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onpopstate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onpopstate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onpopstate,
                    );
                __widl_f_set_onpopstate_HTMLFrameSetElement(self_, onpopstate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstorage_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstorage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onstorage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstorage_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstorage_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstorage_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstorage_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onstorage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onstorage)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onstorage(&self, onstorage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstorage_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstorage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstorage_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstorage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstorage);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstorage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstorage,
                    );
                __widl_f_set_onstorage_HTMLFrameSetElement(self_, onstorage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onunload_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onunload` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn onunload(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onunload_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onunload_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onunload_HTMLFrameSetElement(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlFrameSetElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onunload_HTMLFrameSetElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlFrameSetElement as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlFrameSetElement {
    #[cfg(all(feature = "HtmlFrameSetElement",))]
    #[allow(bad_style)]
    #[doc = "The `onunload` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFrameSetElement/onunload)\n\n*This API requires the following crate features to be activated: `HtmlFrameSetElement`*"]
    #[allow(clippy::all)]
    pub fn set_onunload(&self, onunload: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "HtmlFrameSetElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onunload_HTMLFrameSetElement(
                self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onunload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onunload_HTMLFrameSetElement(
            self_: <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onunload: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onunload);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlFrameSetElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onunload =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onunload,
                    );
                __widl_f_set_onunload_HTMLFrameSetElement(self_, onunload)
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
pub static __WASM_BINDGEN_GENERATED_875aefead2187e94: [u8; 3621usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xE3\r\0\0\0\0!\0\0\x02\x13HTMLFrameSetElement%__widl_instanceof_HTMLFrameSetElement\0\0\0\0!__widl_f_cols_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x04cols\x01\x01\x05self_\x04cols\0\0\0%__widl_f_set_cols_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x04cols\x01\x02\x05self_\x04cols\x04cols\0\0\0!__widl_f_rows_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x04rows\x01\x01\x05self_\x04rows\0\0\0%__widl_f_set_rows_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x04rows\x01\x02\x05self_\x04rows\x04rows\0\0\0)__widl_f_onafterprint_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x0Conafterprint\x01\x01\x05self_\x0Conafterprint\0\0\0-__widl_f_set_onafterprint_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x0Conafterprint\x01\x02\x05self_\x0Conafterprint\x0Conafterprint\0\0\0*__widl_f_onbeforeprint_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\ronbeforeprint\x01\x01\x05self_\ronbeforeprint\0\0\0.__widl_f_set_onbeforeprint_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\ronbeforeprint\x01\x02\x05self_\ronbeforeprint\ronbeforeprint\0\0\0+__widl_f_onbeforeunload_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x0Eonbeforeunload\x01\x01\x05self_\x0Eonbeforeunload\0\0\0/__widl_f_set_onbeforeunload_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x0Eonbeforeunload\x01\x02\x05self_\x0Eonbeforeunload\x0Eonbeforeunload\0\0\0)__widl_f_onhashchange_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x0Conhashchange\x01\x01\x05self_\x0Conhashchange\0\0\0-__widl_f_set_onhashchange_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x0Conhashchange\x01\x02\x05self_\x0Conhashchange\x0Conhashchange\0\0\0-__widl_f_onlanguagechange_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x10onlanguagechange\x01\x01\x05self_\x10onlanguagechange\0\0\01__widl_f_set_onlanguagechange_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x10onlanguagechange\x01\x02\x05self_\x10onlanguagechange\x10onlanguagechange\0\0\0&__widl_f_onmessage_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0*__widl_f_set_onmessage_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0+__widl_f_onmessageerror_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x0Eonmessageerror\x01\x01\x05self_\x0Eonmessageerror\0\0\0/__widl_f_set_onmessageerror_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x0Eonmessageerror\x01\x02\x05self_\x0Eonmessageerror\x0Eonmessageerror\0\0\0&__widl_f_onoffline_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\tonoffline\x01\x01\x05self_\tonoffline\0\0\0*__widl_f_set_onoffline_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\tonoffline\x01\x02\x05self_\tonoffline\tonoffline\0\0\0%__widl_f_ononline_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x08ononline\x01\x01\x05self_\x08ononline\0\0\0)__widl_f_set_ononline_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x08ononline\x01\x02\x05self_\x08ononline\x08ononline\0\0\0'__widl_f_onpagehide_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\nonpagehide\x01\x01\x05self_\nonpagehide\0\0\0+__widl_f_set_onpagehide_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\nonpagehide\x01\x02\x05self_\nonpagehide\nonpagehide\0\0\0'__widl_f_onpageshow_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\nonpageshow\x01\x01\x05self_\nonpageshow\0\0\0+__widl_f_set_onpageshow_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\nonpageshow\x01\x02\x05self_\nonpageshow\nonpageshow\0\0\0'__widl_f_onpopstate_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\nonpopstate\x01\x01\x05self_\nonpopstate\0\0\0+__widl_f_set_onpopstate_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\nonpopstate\x01\x02\x05self_\nonpopstate\nonpopstate\0\0\0&__widl_f_onstorage_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\tonstorage\x01\x01\x05self_\tonstorage\0\0\0*__widl_f_set_onstorage_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\tonstorage\x01\x02\x05self_\tonstorage\tonstorage\0\0\0%__widl_f_onunload_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x01\x08onunload\x01\x01\x05self_\x08onunload\0\0\0)__widl_f_set_onunload_HTMLFrameSetElement\0\0\0\x01\x13HTMLFrameSetElement\x01\0\x02\x08onunload\x01\x02\x05self_\x08onunload\x08onunload\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
