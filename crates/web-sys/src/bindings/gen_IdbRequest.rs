use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbRequest {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for IdbRequest {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbRequest {
        #[inline]
        fn from(obj: JsValue) -> IdbRequest {
            IdbRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbRequest> for IdbRequest {
        #[inline]
        fn as_ref(&self) -> &IdbRequest {
            self
        }
    }
    impl From<IdbRequest> for JsValue {
        #[inline]
        fn from(obj: IdbRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbRequest> for EventTarget {
    #[inline]
    fn from(obj: IdbRequest) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbRequest {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_result_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `result` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn result(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_result_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_result_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_result_IDBRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "DomException", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<DomException> as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "DomException", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error)\n\n*This API requires the following crate features to be activated: `DomException`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> Result<Option<DomException>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "DomException", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_IDBRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `source` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn source(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_source_IDBRequest(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbRequest", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transaction_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<IdbTransaction> as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `transaction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction)\n\n*This API requires the following crate features to be activated: `IdbRequest`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn transaction(&self) -> Option<IdbTransaction> {
        #[cfg(all(feature = "IdbRequest", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transaction_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbTransaction> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transaction_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbTransaction> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_transaction_IDBRequest(self_)
            };
            <Option<IdbTransaction> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbRequest", feature = "IdbRequestReadyState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <IdbRequestReadyState as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest", feature = "IdbRequestReadyState",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)\n\n*This API requires the following crate features to be activated: `IdbRequest`, `IdbRequestReadyState`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> IdbRequestReadyState {
        #[cfg(all(feature = "IdbRequest", feature = "IdbRequestReadyState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequestReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequestReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_IDBRequest(self_)
            };
            <IdbRequestReadyState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsuccess_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onsuccess` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn onsuccess(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsuccess_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsuccess_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsuccess_IDBRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsuccess_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onsuccess` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onsuccess(&self, onsuccess: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsuccess_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsuccess: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsuccess_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsuccess =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsuccess,
                    );
                __widl_f_set_onsuccess_IDBRequest(self_, onsuccess)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_IDBRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_IDBRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbRequest {
    #[cfg(all(feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)\n\n*This API requires the following crate features to be activated: `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_IDBRequest(
                self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_IDBRequest(
            self_: <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_IDBRequest(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_3bc95f77f67441a9: [u8; 854usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x14\x03\0\0\0\0\n\0\0\x02\nIDBRequest\x1C__widl_instanceof_IDBRequest\0\0\0\0\x1A__widl_f_result_IDBRequest\x01\0\0\x01\nIDBRequest\x01\0\x01\x06result\x01\x01\x05self_\x06result\0\0\0\x19__widl_f_error_IDBRequest\x01\0\0\x01\nIDBRequest\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\x1A__widl_f_source_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x01\x06source\x01\x01\x05self_\x06source\0\0\0\x1F__widl_f_transaction_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x01\x0Btransaction\x01\x01\x05self_\x0Btransaction\0\0\0\x1F__widl_f_ready_state_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1D__widl_f_onsuccess_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x01\tonsuccess\x01\x01\x05self_\tonsuccess\0\0\0!__widl_f_set_onsuccess_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x02\tonsuccess\x01\x02\x05self_\tonsuccess\tonsuccess\0\0\0\x1B__widl_f_onerror_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\x1F__widl_f_set_onerror_IDBRequest\0\0\0\x01\nIDBRequest\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
