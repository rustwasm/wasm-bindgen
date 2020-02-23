use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `DOMRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct DomRequest {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_DomRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for DomRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(68u32);
            inform(79u32);
            inform(77u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for DomRequest {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for DomRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for DomRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a DomRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for DomRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            DomRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for DomRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a DomRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for DomRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<DomRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(DomRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for DomRequest {
        #[inline]
        fn from(obj: JsValue) -> DomRequest {
            DomRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for DomRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<DomRequest> for DomRequest {
        #[inline]
        fn as_ref(&self) -> &DomRequest {
            self
        }
    }
    impl From<DomRequest> for JsValue {
        #[inline]
        fn from(obj: DomRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for DomRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_DOMRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_DOMRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_DOMRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            DomRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const DomRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<DomRequest> for EventTarget {
    #[inline]
    fn from(obj: DomRequest) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for DomRequest {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<DomRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: DomRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for DomRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_then_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRequest as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `then()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn then(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_then_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_then_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_then_DOMRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_then_with_fulfill_callback_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `then()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn then_with_fulfill_callback(
        &self,
        fulfill_callback: Option<&::js_sys::Function>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_then_with_fulfill_callback_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fulfill_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_then_with_fulfill_callback_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fulfill_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(fulfill_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fulfill_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        fulfill_callback,
                    );
                __widl_f_then_with_fulfill_callback_DOMRequest(self_, fulfill_callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_then_with_fulfill_callback_and_reject_callback_DOMRequest(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `then()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn then_with_fulfill_callback_and_reject_callback(
        &self,
        fulfill_callback: Option<&::js_sys::Function>,
        reject_callback: Option<&::js_sys::Function>,
    ) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_then_with_fulfill_callback_and_reject_callback_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fulfill_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                reject_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_then_with_fulfill_callback_and_reject_callback_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fulfill_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            reject_callback : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(fulfill_callback);
            drop(reject_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fulfill_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        fulfill_callback,
                    );
                let reject_callback =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        reject_callback,
                    );
                __widl_f_then_with_fulfill_callback_and_reject_callback_DOMRequest(
                    self_,
                    fulfill_callback,
                    reject_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomRequest", feature = "DomRequestReadyState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRequest as WasmDescribe>::describe();
    <DomRequestReadyState as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest", feature = "DomRequestReadyState",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/readyState)\n\n*This API requires the following crate features to be activated: `DomRequest`, `DomRequestReadyState`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> DomRequestReadyState {
        #[cfg(all(feature = "DomRequest", feature = "DomRequestReadyState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomRequestReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomRequestReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_DOMRequest(self_)
            };
            <DomRequestReadyState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRequest as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/result)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> ::wasm_bindgen::JsValue {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_DOMRequest(self_)
            };
            <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException", feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<DomException> as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomException", feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/error)\n\n*This API requires the following crate features to be activated: `DomException`, `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> Option<DomException> {
        #[cfg(all(feature = "DomException", feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_DOMRequest(self_)
            };
            <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsuccess_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onsuccess` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn onsuccess(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsuccess_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsuccess_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsuccess_DOMRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsuccess_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onsuccess` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onsuccess(&self, onsuccess: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsuccess_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsuccess: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsuccess_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsuccess: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsuccess);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsuccess =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsuccess,
                    );
                __widl_f_set_onsuccess_DOMRequest(self_, onsuccess)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_DOMRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_DOMRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&DomRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl DomRequest {
    #[cfg(all(feature = "DomRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)\n\n*This API requires the following crate features to be activated: `DomRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "DomRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_DOMRequest(
                self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_DOMRequest(
            self_: <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&DomRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_DOMRequest(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_68da5649abeff127: [u8; 990usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x9C\x03\0\0\0\0\x0B\0\0\x02\nDOMRequest\x1C__widl_instanceof_DOMRequest\0\0\0\0\x18__widl_f_then_DOMRequest\x01\0\0\x01\nDOMRequest\x01\0\0\x01\x01\x05self_\x04then\0\0\0.__widl_f_then_with_fulfill_callback_DOMRequest\x01\0\0\x01\nDOMRequest\x01\0\0\x01\x02\x05self_\x10fulfill_callback\x04then\0\0\0B__widl_f_then_with_fulfill_callback_and_reject_callback_DOMRequest\x01\0\0\x01\nDOMRequest\x01\0\0\x01\x03\x05self_\x10fulfill_callback\x0Freject_callback\x04then\0\0\0\x1F__widl_f_ready_state_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1A__widl_f_result_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\x19__widl_f_error_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\x1D__widl_f_onsuccess_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x01\tonsuccess\x01\x01\x05self_\tonsuccess\0\0\0!__widl_f_set_onsuccess_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x02\tonsuccess\x01\x02\x05self_\tonsuccess\tonsuccess\0\0\0\x1B__widl_f_onerror_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1F__widl_f_set_onerror_DOMRequest\0\0\0\x01\nDOMRequest\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
