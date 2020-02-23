use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBTransaction` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbTransaction {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbTransaction: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbTransaction {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(110u32);
            inform(115u32);
            inform(97u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for IdbTransaction {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbTransaction {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbTransaction {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbTransaction {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbTransaction {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbTransaction {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbTransaction {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbTransaction {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbTransaction {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbTransaction>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbTransaction {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbTransaction {
        #[inline]
        fn from(obj: JsValue) -> IdbTransaction {
            IdbTransaction { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbTransaction {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbTransaction> for IdbTransaction {
        #[inline]
        fn as_ref(&self) -> &IdbTransaction {
            self
        }
    }
    impl From<IdbTransaction> for JsValue {
        #[inline]
        fn from(obj: IdbTransaction) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbTransaction {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBTransaction(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBTransaction(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBTransaction(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbTransaction { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbTransaction) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbTransaction> for EventTarget {
    #[inline]
    fn from(obj: IdbTransaction) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbTransaction {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbTransaction> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbTransaction) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbTransaction {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/abort)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_IDBTransaction(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbObjectStore", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_object_store_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbObjectStore as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbObjectStore", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `objectStore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStore)\n\n*This API requires the following crate features to be activated: `IdbObjectStore`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn object_store(&self, name: &str) -> Result<IdbObjectStore, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbObjectStore", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_object_store_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_object_store_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_object_store_IDBTransaction(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mode_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <IdbTransactionMode as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/mode)\n\n*This API requires the following crate features to be activated: `IdbTransaction`, `IdbTransactionMode`*"]
    #[allow(clippy::all)]
    pub fn mode(&self) -> Result<IdbTransactionMode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbTransaction", feature = "IdbTransactionMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mode_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbTransactionMode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mode_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbTransactionMode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mode_IDBTransaction(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbTransactionMode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_db_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <IdbDatabase as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `db` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/db)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn db(&self) -> IdbDatabase {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_db_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbDatabase as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_db_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbDatabase as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_db_IDBTransaction(self_)
            };
            <IdbDatabase as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomException", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<DomException> as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "DomException", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/error)\n\n*This API requires the following crate features to be activated: `DomException`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> Option<DomException> {
        #[cfg(all(feature = "DomException", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_IDBTransaction(self_)
            };
            <Option<DomException> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_IDBTransaction(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onabort)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onabort);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_IDBTransaction(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncomplete_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `oncomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn oncomplete(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncomplete_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncomplete_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncomplete_IDBTransaction(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncomplete_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `oncomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/oncomplete)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn set_oncomplete(&self, oncomplete: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncomplete_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncomplete : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncomplete_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncomplete: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncomplete);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncomplete =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncomplete,
                    );
                __widl_f_set_oncomplete_IDBTransaction(self_, oncomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_IDBTransaction(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/onerror)\n\n*This API requires the following crate features to be activated: `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_IDBTransaction(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "DomStringList", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_object_store_names_IDBTransaction() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbTransaction as WasmDescribe>::describe();
    <DomStringList as WasmDescribe>::describe();
}
impl IdbTransaction {
    #[cfg(all(feature = "DomStringList", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `objectStoreNames` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBTransaction/objectStoreNames)\n\n*This API requires the following crate features to be activated: `DomStringList`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn object_store_names(&self) -> DomStringList {
        #[cfg(all(feature = "DomStringList", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_object_store_names_IDBTransaction(
                self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_object_store_names_IDBTransaction(
            self_: <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbTransaction as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_object_store_names_IDBTransaction(self_)
            };
            <DomStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_09d7d3732c7a9cab: [u8; 1179usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}Y\x04\0\0\0\0\r\0\0\x02\x0EIDBTransaction __widl_instanceof_IDBTransaction\0\0\0\0\x1D__widl_f_abort_IDBTransaction\x01\0\0\x01\x0EIDBTransaction\x01\0\0\x01\x01\x05self_\x05abort\0\0\0$__widl_f_object_store_IDBTransaction\x01\0\0\x01\x0EIDBTransaction\x01\0\0\x01\x02\x05self_\x04name\x0BobjectStore\0\0\0\x1C__widl_f_mode_IDBTransaction\x01\0\0\x01\x0EIDBTransaction\x01\0\x01\x04mode\x01\x01\x05self_\x04mode\0\0\0\x1A__widl_f_db_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x01\x02db\x01\x01\x05self_\x02db\0\0\0\x1D__widl_f_error_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0\x1F__widl_f_onabort_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0#__widl_f_set_onabort_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\"__widl_f_oncomplete_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x01\noncomplete\x01\x01\x05self_\noncomplete\0\0\0&__widl_f_set_oncomplete_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x02\noncomplete\x01\x02\x05self_\noncomplete\noncomplete\0\0\0\x1F__widl_f_onerror_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0#__widl_f_set_onerror_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0*__widl_f_object_store_names_IDBTransaction\0\0\0\x01\x0EIDBTransaction\x01\0\x01\x10objectStoreNames\x01\x01\x05self_\x10objectStoreNames\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
