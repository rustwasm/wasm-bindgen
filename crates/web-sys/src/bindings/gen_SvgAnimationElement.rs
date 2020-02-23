use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SVGAnimationElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SvgAnimationElement {
    obj: SvgElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SvgAnimationElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SvgAnimationElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(83u32);
            inform(86u32);
            inform(71u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for SvgAnimationElement {
        type Target = SvgElement;
        #[inline]
        fn deref(&self) -> &SvgElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for SvgAnimationElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SvgAnimationElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SvgAnimationElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SvgAnimationElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SvgAnimationElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SvgAnimationElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SvgAnimationElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SvgAnimationElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SvgAnimationElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SvgAnimationElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SvgAnimationElement {
        #[inline]
        fn from(obj: JsValue) -> SvgAnimationElement {
            SvgAnimationElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SvgAnimationElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SvgAnimationElement> for SvgAnimationElement {
        #[inline]
        fn as_ref(&self) -> &SvgAnimationElement {
            self
        }
    }
    impl From<SvgAnimationElement> for JsValue {
        #[inline]
        fn from(obj: SvgAnimationElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SvgAnimationElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SVGAnimationElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SVGAnimationElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SVGAnimationElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SvgAnimationElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SvgAnimationElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SvgAnimationElement> for SvgElement {
    #[inline]
    fn from(obj: SvgAnimationElement) -> SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<SvgElement> for SvgAnimationElement {
    #[inline]
    fn as_ref(&self) -> &SvgElement {
        use wasm_bindgen::JsCast;
        SvgElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimationElement> for Element {
    #[inline]
    fn from(obj: SvgAnimationElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for SvgAnimationElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimationElement> for Node {
    #[inline]
    fn from(obj: SvgAnimationElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for SvgAnimationElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimationElement> for EventTarget {
    #[inline]
    fn from(obj: SvgAnimationElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SvgAnimationElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SvgAnimationElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: SvgAnimationElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SvgAnimationElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_begin_element_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `beginElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn begin_element(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_begin_element_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_begin_element_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_begin_element_SVGAnimationElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_begin_element_at_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `beginElementAt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElementAt)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn begin_element_at(&self, offset: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_begin_element_at_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_begin_element_at_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_begin_element_at_SVGAnimationElement(self_, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_element_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `endElement()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn end_element(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_element_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_element_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_end_element_SVGAnimationElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_element_at_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `endElementAt()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElementAt)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn end_element_at(&self, offset: f32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_element_at_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                offset: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_element_at_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            offset: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let offset = <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(offset);
                __widl_f_end_element_at_SVGAnimationElement(self_, offset)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_current_time_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `getCurrentTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getCurrentTime)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn get_current_time(&self) -> f32 {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_current_time_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_current_time_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_current_time_SVGAnimationElement(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_simple_duration_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `getSimpleDuration()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getSimpleDuration)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn get_simple_duration(&self) -> Result<f32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_simple_duration_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_simple_duration_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_simple_duration_SVGAnimationElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_start_time_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `getStartTime()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getStartTime)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn get_start_time(&self) -> Result<f32, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_start_time_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_start_time_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_start_time_SVGAnimationElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement", feature = "SvgElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_element_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <Option<SvgElement> as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement", feature = "SvgElement",))]
    #[allow(bad_style)]
    #[doc = "The `targetElement` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/targetElement)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgElement`*"]
    #[allow(clippy::all)]
    pub fn target_element(&self) -> Option<SvgElement> {
        #[cfg(all(feature = "SvgAnimationElement", feature = "SvgElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_element_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_element_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_element_SVGAnimationElement(self_)
            };
            <Option<SvgElement> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_has_extension_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement",))]
    #[allow(bad_style)]
    #[doc = "The `hasExtension()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/hasExtension)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
    #[allow(clippy::all)]
    pub fn has_extension(&self, extension: &str) -> bool {
        #[cfg(all(feature = "SvgAnimationElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_has_extension_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                extension: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_has_extension_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            extension: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(extension);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let extension = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(extension);
                __widl_f_has_extension_SVGAnimationElement(self_, extension)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_features_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
    #[allow(bad_style)]
    #[doc = "The `requiredFeatures` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredFeatures)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    #[allow(clippy::all)]
    pub fn required_features(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_features_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_features_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_features_SVGAnimationElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_required_extensions_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
    #[allow(bad_style)]
    #[doc = "The `requiredExtensions` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredExtensions)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    #[allow(clippy::all)]
    pub fn required_extensions(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_required_extensions_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_required_extensions_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_required_extensions_SVGAnimationElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_system_language_SVGAnimationElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SvgAnimationElement as WasmDescribe>::describe();
    <SvgStringList as WasmDescribe>::describe();
}
impl SvgAnimationElement {
    #[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
    #[allow(bad_style)]
    #[doc = "The `systemLanguage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/systemLanguage)\n\n*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
    #[allow(clippy::all)]
    pub fn system_language(&self) -> SvgStringList {
        #[cfg(all(feature = "SvgAnimationElement", feature = "SvgStringList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_system_language_SVGAnimationElement(
                self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_system_language_SVGAnimationElement(
            self_: <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&SvgAnimationElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_system_language_SVGAnimationElement(self_)
            };
            <SvgStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_46462965c861e0f6: [u8; 1429usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}S\x05\0\0\0\0\r\0\0\x02\x13SVGAnimationElement%__widl_instanceof_SVGAnimationElement\0\0\0\0*__widl_f_begin_element_SVGAnimationElement\x01\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x01\x05self_\x0CbeginElement\0\0\0-__widl_f_begin_element_at_SVGAnimationElement\x01\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x02\x05self_\x06offset\x0EbeginElementAt\0\0\0(__widl_f_end_element_SVGAnimationElement\x01\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x01\x05self_\nendElement\0\0\0+__widl_f_end_element_at_SVGAnimationElement\x01\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x02\x05self_\x06offset\x0CendElementAt\0\0\0-__widl_f_get_current_time_SVGAnimationElement\0\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x01\x05self_\x0EgetCurrentTime\0\0\00__widl_f_get_simple_duration_SVGAnimationElement\x01\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x01\x05self_\x11getSimpleDuration\0\0\0+__widl_f_get_start_time_SVGAnimationElement\x01\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x01\x05self_\x0CgetStartTime\0\0\0+__widl_f_target_element_SVGAnimationElement\0\0\0\x01\x13SVGAnimationElement\x01\0\x01\rtargetElement\x01\x01\x05self_\rtargetElement\0\0\0*__widl_f_has_extension_SVGAnimationElement\0\0\0\x01\x13SVGAnimationElement\x01\0\0\x01\x02\x05self_\textension\x0ChasExtension\0\0\0.__widl_f_required_features_SVGAnimationElement\0\0\0\x01\x13SVGAnimationElement\x01\0\x01\x10requiredFeatures\x01\x01\x05self_\x10requiredFeatures\0\0\00__widl_f_required_extensions_SVGAnimationElement\0\0\0\x01\x13SVGAnimationElement\x01\0\x01\x12requiredExtensions\x01\x01\x05self_\x12requiredExtensions\0\0\0,__widl_f_system_language_SVGAnimationElement\0\0\0\x01\x13SVGAnimationElement\x01\0\x01\x0EsystemLanguage\x01\x01\x05self_\x0EsystemLanguage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
