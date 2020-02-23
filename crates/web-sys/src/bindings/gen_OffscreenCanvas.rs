use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OffscreenCanvas` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OffscreenCanvas {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OffscreenCanvas: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OffscreenCanvas {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(79u32);
            inform(102u32);
            inform(102u32);
            inform(115u32);
            inform(99u32);
            inform(114u32);
            inform(101u32);
            inform(101u32);
            inform(110u32);
            inform(67u32);
            inform(97u32);
            inform(110u32);
            inform(118u32);
            inform(97u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for OffscreenCanvas {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for OffscreenCanvas {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OffscreenCanvas {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OffscreenCanvas {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OffscreenCanvas {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OffscreenCanvas {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OffscreenCanvas {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OffscreenCanvas {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OffscreenCanvas {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OffscreenCanvas>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OffscreenCanvas {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OffscreenCanvas {
        #[inline]
        fn from(obj: JsValue) -> OffscreenCanvas {
            OffscreenCanvas { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OffscreenCanvas {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OffscreenCanvas> for OffscreenCanvas {
        #[inline]
        fn as_ref(&self) -> &OffscreenCanvas {
            self
        }
    }
    impl From<OffscreenCanvas> for JsValue {
        #[inline]
        fn from(obj: OffscreenCanvas) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OffscreenCanvas {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OffscreenCanvas(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OffscreenCanvas(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OffscreenCanvas(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OffscreenCanvas { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OffscreenCanvas) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OffscreenCanvas> for EventTarget {
    #[inline]
    fn from(obj: OffscreenCanvas) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for OffscreenCanvas {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OffscreenCanvas> for ::js_sys::Object {
    #[inline]
    fn from(obj: OffscreenCanvas) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OffscreenCanvas {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <OffscreenCanvas as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `new OffscreenCanvas(..)` constructor, creating a new instance of `OffscreenCanvas`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/OffscreenCanvas)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn new(width: u32, height: u32) -> Result<OffscreenCanvas, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_OffscreenCanvas(
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OffscreenCanvas as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_OffscreenCanvas(
            width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OffscreenCanvas as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(width);
            drop(height);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_new_OffscreenCanvas(width, height)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OffscreenCanvas as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_context_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `getContext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn get_context(
        &self,
        context_id: &str,
    ) -> Result<Option<::js_sys::Object>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_context_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_context_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_id);
                __widl_f_get_context_OffscreenCanvas(self_, context_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_context_with_context_options_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `getContext()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn get_context_with_context_options(
        &self,
        context_id: &str,
        context_options: &::wasm_bindgen::JsValue,
    ) -> Result<Option<::js_sys::Object>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_context_with_context_options_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                context_options : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_context_with_context_options_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let context_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context_id);
                let context_options =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        context_options,
                    );
                __widl_f_get_context_with_context_options_OffscreenCanvas(
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
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_blob_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `toBlob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/toBlob)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn to_blob(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_blob_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_blob_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_to_blob_OffscreenCanvas(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_blob_with_type_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `toBlob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/toBlob)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn to_blob_with_type(
        &self,
        type_: &str,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_blob_with_type_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_blob_with_type_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_to_blob_with_type_OffscreenCanvas(self_, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_to_blob_with_type_and_encoder_options_OffscreenCanvas(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `toBlob()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/toBlob)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn to_blob_with_type_and_encoder_options(
        &self,
        type_: &str,
        encoder_options: &::wasm_bindgen::JsValue,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_to_blob_with_type_and_encoder_options_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoder_options : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_to_blob_with_type_and_encoder_options_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoder_options: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                let encoder_options =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        encoder_options,
                    );
                __widl_f_to_blob_with_type_and_encoder_options_OffscreenCanvas(
                    self_,
                    type_,
                    encoder_options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "ImageBitmap", feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transfer_to_image_bitmap_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <ImageBitmap as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "ImageBitmap", feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `transferToImageBitmap()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/transferToImageBitmap)\n\n*This API requires the following crate features to be activated: `ImageBitmap`, `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn transfer_to_image_bitmap(&self) -> Result<ImageBitmap, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ImageBitmap", feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transfer_to_image_bitmap_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ImageBitmap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transfer_to_image_bitmap_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ImageBitmap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transfer_to_image_bitmap_OffscreenCanvas(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ImageBitmap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_width_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `width` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn width(&self) -> u32 {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_width_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_width_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_width_OffscreenCanvas(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_width_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `width` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn set_width(&self, width: u32) {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_width_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                width: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_width_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let width = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(width);
                __widl_f_set_width_OffscreenCanvas(self_, width)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_height_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `height` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn height(&self) -> u32 {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_height_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_height_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_height_OffscreenCanvas(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OffscreenCanvas",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_height_OffscreenCanvas() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OffscreenCanvas as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OffscreenCanvas {
    #[cfg(all(feature = "OffscreenCanvas",))]
    #[allow(bad_style)]
    #[doc = "The `height` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)\n\n*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
    #[allow(clippy::all)]
    pub fn set_height(&self, height: u32) {
        #[cfg(all(feature = "OffscreenCanvas",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_height_OffscreenCanvas(
                self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                height: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_height_OffscreenCanvas(
            self_: <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&OffscreenCanvas as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let height = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(height);
                __widl_f_set_height_OffscreenCanvas(self_, height)
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
pub static __WASM_BINDGEN_GENERATED_f148ec69a038e5b0: [u8; 1185usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}_\x04\0\0\0\0\x0C\0\0\x02\x0FOffscreenCanvas!__widl_instanceof_OffscreenCanvas\0\0\0\0\x1C__widl_f_new_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\0\x01\x02\x05width\x06height\x03new\0\0\0$__widl_f_get_context_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\x01\0\0\x01\x02\x05self_\ncontext_id\ngetContext\0\0\09__widl_f_get_context_with_context_options_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\x01\0\0\x01\x03\x05self_\ncontext_id\x0Fcontext_options\ngetContext\0\0\0 __widl_f_to_blob_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\x01\0\0\x01\x01\x05self_\x06toBlob\0\0\0*__widl_f_to_blob_with_type_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\x01\0\0\x01\x02\x05self_\x05type_\x06toBlob\0\0\0>__widl_f_to_blob_with_type_and_encoder_options_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\x01\0\0\x01\x03\x05self_\x05type_\x0Fencoder_options\x06toBlob\0\0\01__widl_f_transfer_to_image_bitmap_OffscreenCanvas\x01\0\0\x01\x0FOffscreenCanvas\x01\0\0\x01\x01\x05self_\x15transferToImageBitmap\0\0\0\x1E__widl_f_width_OffscreenCanvas\0\0\0\x01\x0FOffscreenCanvas\x01\0\x01\x05width\x01\x01\x05self_\x05width\0\0\0\"__widl_f_set_width_OffscreenCanvas\0\0\0\x01\x0FOffscreenCanvas\x01\0\x02\x05width\x01\x02\x05self_\x05width\x05width\0\0\0\x1F__widl_f_height_OffscreenCanvas\0\0\0\x01\x0FOffscreenCanvas\x01\0\x01\x06height\x01\x01\x05self_\x06height\0\0\0#__widl_f_set_height_OffscreenCanvas\0\0\0\x01\x0FOffscreenCanvas\x01\0\x02\x06height\x01\x02\x05self_\x06height\x06height\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
