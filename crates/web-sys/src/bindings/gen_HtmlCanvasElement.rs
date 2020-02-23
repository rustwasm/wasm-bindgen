use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `HTMLCanvasElement` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct HtmlCanvasElement {
    obj: HtmlElement,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_HtmlCanvasElement: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for HtmlCanvasElement {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(17u32);
            inform(72u32);
            inform(84u32);
            inform(77u32);
            inform(76u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(118u32);
            inform(97u32);
            inform(115u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for HtmlCanvasElement {
        type Target = HtmlElement;
        #[inline]
        fn deref(&self) -> &HtmlElement {
            &self.obj
        }
    }
    impl IntoWasmAbi for HtmlCanvasElement {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for HtmlCanvasElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a HtmlCanvasElement {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for HtmlCanvasElement {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            HtmlCanvasElement {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for HtmlCanvasElement {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a HtmlCanvasElement {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for HtmlCanvasElement {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<HtmlCanvasElement>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(HtmlCanvasElement {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for HtmlCanvasElement {
        #[inline]
        fn from(obj: JsValue) -> HtmlCanvasElement {
            HtmlCanvasElement { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for HtmlCanvasElement {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<HtmlCanvasElement> for HtmlCanvasElement {
        #[inline]
        fn as_ref(&self) -> &HtmlCanvasElement {
            self
        }
    }
    impl From<HtmlCanvasElement> for JsValue {
        #[inline]
        fn from(obj: HtmlCanvasElement) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for HtmlCanvasElement {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_HTMLCanvasElement(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_HTMLCanvasElement(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_HTMLCanvasElement(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            HtmlCanvasElement { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const HtmlCanvasElement) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<HtmlCanvasElement> for HtmlElement {
    #[inline]
    fn from(obj: HtmlCanvasElement) -> HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<HtmlElement> for HtmlCanvasElement {
    #[inline]
    fn as_ref(&self) -> &HtmlElement {
        use wasm_bindgen::JsCast;
        HtmlElement::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlCanvasElement> for Element {
    #[inline]
    fn from(obj: HtmlCanvasElement) -> Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Element> for HtmlCanvasElement {
    #[inline]
    fn as_ref(&self) -> &Element {
        use wasm_bindgen::JsCast;
        Element::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlCanvasElement> for Node {
    #[inline]
    fn from(obj: HtmlCanvasElement) -> Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Node> for HtmlCanvasElement {
    #[inline]
    fn as_ref(&self) -> &Node {
        use wasm_bindgen::JsCast;
        Node::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlCanvasElement> for EventTarget {
    #[inline]
    fn from(obj: HtmlCanvasElement) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for HtmlCanvasElement {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<HtmlCanvasElement> for ::js_sys::Object {
    #[inline]
    fn from(obj: HtmlCanvasElement) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for HtmlCanvasElement {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_context_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `getContext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn get_context(
        &self,
        context_id: &str,
    ) -> Result<Option<::js_sys::Object>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_context_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_context_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(context_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_id);
                __widl_f_get_context_HTMLCanvasElement(self_, context_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_context_with_context_options_HTMLCanvasElement()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `getContext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn get_context_with_context_options(
        &self,
        context_id: &str,
        context_options: &::wasm_bindgen::JsValue,
    ) -> Result<Option<::js_sys::Object>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_context_with_context_options_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_options : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_context_with_context_options_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            context_options: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(context_id);
            drop(context_options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_id);
                let context_options =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        context_options,
                    );
                __widl_f_get_context_with_context_options_HTMLCanvasElement(
                    self_,
                    context_id,
                    context_options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_blob_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `toBlob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn to_blob(&self, callback: &::js_sys::Function) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_blob_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_blob_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                __widl_f_to_blob_HTMLCanvasElement(self_, callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_blob_with_type_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `toBlob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn to_blob_with_type(
        &self,
        callback: &::js_sys::Function,
        type_: &str,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_blob_with_type_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_blob_with_type_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(callback);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_to_blob_with_type_HTMLCanvasElement(self_, callback, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_blob_with_type_and_encoder_options_HTMLCanvasElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `toBlob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn to_blob_with_type_and_encoder_options(
        &self,
        callback: &::js_sys::Function,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_blob_with_type_and_encoder_options_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoder_options : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_blob_with_type_and_encoder_options_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoder_options: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(callback);
            drop(type_);
            drop(encoder_options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let encoder_options =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        encoder_options,
                    );
                __widl_f_to_blob_with_type_and_encoder_options_HTMLCanvasElement(
                    self_,
                    callback,
                    type_,
                    encoder_options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_data_url_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `toDataURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn to_data_url(&self) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_data_url_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_data_url_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_data_url_HTMLCanvasElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_data_url_with_type_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `toDataURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn to_data_url_with_type(&self, type_: &str) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_data_url_with_type_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_data_url_with_type_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_to_data_url_with_type_HTMLCanvasElement(self_, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_data_url_with_type_and_encoder_options_HTMLCanvasElement(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `toDataURL()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn to_data_url_with_type_and_encoder_options(
        &self,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<String, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_data_url_with_type_and_encoder_options_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoder_options : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_data_url_with_type_and_encoder_options_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoder_options: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(type_);
            drop(encoder_options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let encoder_options =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        encoder_options,
                    );
                __widl_f_to_data_url_with_type_and_encoder_options_HTMLCanvasElement(
                    self_,
                    type_,
                    encoder_options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<String as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement", feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transfer_control_to_offscreen_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <OffscreenCanvas as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement", feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `transferControlToOffscreen()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/transferControlToOffscreen)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn transfer_control_to_offscreen(
        &self,
    ) -> Result<OffscreenCanvas, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "HtmlCanvasElement", feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transfer_control_to_offscreen_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OffscreenCanvas as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transfer_control_to_offscreen_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OffscreenCanvas as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transfer_control_to_offscreen_HTMLCanvasElement(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OffscreenCanvas as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_HTMLCanvasElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: u32) {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_HTMLCanvasElement(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_HTMLCanvasElement(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "HtmlCanvasElement",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_HTMLCanvasElement() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&HtmlCanvasElement as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl HtmlCanvasElement {
    #[cfg(all(feature = "HtmlCanvasElement",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)\n\n*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: u32) {
        #[cfg(all(feature = "HtmlCanvasElement",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_HTMLCanvasElement(
                self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_HTMLCanvasElement(
            self_: <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&HtmlCanvasElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_HTMLCanvasElement(self_, height)
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
pub static __WASM_BINDGEN_GENERATED_f913950f810bbe8f: [u8; 1517usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xAB\x05\0\0\0\0\x0E\0\0\x02\x11HTMLCanvasElement#__widl_instanceof_HTMLCanvasElement\0\0\0\0&__widl_f_get_context_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x02\x05self_\ncontext_id\ngetContext\0\0\0;__widl_f_get_context_with_context_options_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x03\x05self_\ncontext_id\x0Fcontext_options\ngetContext\0\0\0\"__widl_f_to_blob_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x02\x05self_\x08callback\x06toBlob\0\0\0,__widl_f_to_blob_with_type_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x03\x05self_\x08callback\x05type_\x06toBlob\0\0\0@__widl_f_to_blob_with_type_and_encoder_options_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x04\x05self_\x08callback\x05type_\x0Fencoder_options\x06toBlob\0\0\0&__widl_f_to_data_url_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x01\x05self_\ttoDataURL\0\0\00__widl_f_to_data_url_with_type_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x02\x05self_\x05type_\ttoDataURL\0\0\0D__widl_f_to_data_url_with_type_and_encoder_options_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x03\x05self_\x05type_\x0Fencoder_options\ttoDataURL\0\0\08__widl_f_transfer_control_to_offscreen_HTMLCanvasElement\x01\0\0\x01\x11HTMLCanvasElement\x01\0\0\x01\x01\x05self_\x1AtransferControlToOffscreen\0\0\0 __widl_f_width_HTMLCanvasElement\0\0\0\x01\x11HTMLCanvasElement\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0$__widl_f_set_width_HTMLCanvasElement\0\0\0\x01\x11HTMLCanvasElement\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0!__widl_f_height_HTMLCanvasElement\0\0\0\x01\x11HTMLCanvasElement\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0%__widl_f_set_height_HTMLCanvasElement\0\0\0\x01\x11HTMLCanvasElement\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
