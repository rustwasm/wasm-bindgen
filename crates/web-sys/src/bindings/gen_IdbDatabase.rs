use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBDatabase` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbDatabase {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbDatabase: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbDatabase {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
            inform(98u32);
            inform(97u32);
            inform(115u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdbDatabase {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbDatabase {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbDatabase {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbDatabase {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbDatabase {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbDatabase {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbDatabase {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbDatabase {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbDatabase {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbDatabase>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbDatabase {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbDatabase {
        #[inline]
        fn from(obj: JsValue) -> IdbDatabase {
            IdbDatabase { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbDatabase {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbDatabase> for IdbDatabase {
        #[inline]
        fn as_ref(&self) -> &IdbDatabase {
            self
        }
    }
    impl From<IdbDatabase> for JsValue {
        #[inline]
        fn from(obj: IdbDatabase) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbDatabase {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBDatabase(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBDatabase(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBDatabase(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbDatabase { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbDatabase) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbDatabase> for EventTarget {
    #[inline]
    fn from(obj: IdbDatabase) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbDatabase {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbDatabase> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbDatabase) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbDatabase {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/close)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_IDBDatabase(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_mutable_file_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `createMutableFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn create_mutable_file(&self, name: &str) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_mutable_file_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_mutable_file_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_create_mutable_file_IDBDatabase(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_mutable_file_with_type_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `createMutableFile()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createMutableFile)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn create_mutable_file_with_type(
        &self,
        name: &str,
        type_: &str,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_mutable_file_with_type_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_mutable_file_with_type_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_create_mutable_file_with_type_IDBDatabase(self_, name, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbObjectStore",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_object_store_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbObjectStore as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbObjectStore",))]
    #[allow(bad_style)]
    #[doc = "The `createObjectStore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`*"]
    #[allow(clippy::all)]
    pub fn create_object_store(
        &self,
        name: &str,
    ) -> Result<IdbObjectStore, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbObjectStore",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_object_store_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_object_store_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_create_object_store_IDBDatabase(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbDatabase",
    feature = "IdbObjectStore",
    feature = "IdbObjectStoreParameters",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_object_store_with_optional_parameters_IDBDatabase(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&IdbObjectStoreParameters as WasmDescribe>::describe();
    <IdbObjectStore as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(
        feature = "IdbDatabase",
        feature = "IdbObjectStore",
        feature = "IdbObjectStoreParameters",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createObjectStore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/createObjectStore)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbObjectStore`, `IdbObjectStoreParameters`*"]
    #[allow(clippy::all)]
    pub fn create_object_store_with_optional_parameters(
        &self,
        name: &str,
        optional_parameters: &IdbObjectStoreParameters,
    ) -> Result<IdbObjectStore, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbDatabase",
            feature = "IdbObjectStore",
            feature = "IdbObjectStoreParameters",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_object_store_with_optional_parameters_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                optional_parameters : < & IdbObjectStoreParameters as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_object_store_with_optional_parameters_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            optional_parameters : < & IdbObjectStoreParameters as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> <IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(name);
            drop(optional_parameters);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                let optional_parameters =
                    <&IdbObjectStoreParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        optional_parameters,
                    );
                __widl_f_create_object_store_with_optional_parameters_IDBDatabase(
                    self_,
                    name,
                    optional_parameters,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbObjectStore as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_object_store_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `deleteObjectStore()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/deleteObjectStore)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn delete_object_store(&self, name: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_object_store_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_object_store_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_delete_object_store_IDBDatabase(self_, name)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transaction_with_str_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbTransaction as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `transaction()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn transaction_with_str(
        &self,
        store_names: &str,
    ) -> Result<IdbTransaction, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transaction_with_str_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                store_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transaction_with_str_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            store_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(store_names);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let store_names =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(store_names);
                __widl_f_transaction_with_str_IDBDatabase(self_, store_names)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transaction_with_str_sequence_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbTransaction as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
    #[allow(bad_style)]
    #[doc = "The `transaction()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`*"]
    #[allow(clippy::all)]
    pub fn transaction_with_str_sequence(
        &self,
        store_names: &::wasm_bindgen::JsValue,
    ) -> Result<IdbTransaction, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbDatabase", feature = "IdbTransaction",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transaction_with_str_sequence_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                store_names: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transaction_with_str_sequence_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            store_names: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(store_names);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let store_names =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        store_names,
                    );
                __widl_f_transaction_with_str_sequence_IDBDatabase(self_, store_names)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbDatabase",
    feature = "IdbTransaction",
    feature = "IdbTransactionMode",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transaction_with_str_and_mode_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <IdbTransactionMode as WasmDescribe>::describe();
    <IdbTransaction as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(
        feature = "IdbDatabase",
        feature = "IdbTransaction",
        feature = "IdbTransactionMode",
    ))]
    #[allow(bad_style)]
    #[doc = "The `transaction()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
    #[allow(clippy::all)]
    pub fn transaction_with_str_and_mode(
        &self,
        store_names: &str,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbDatabase",
            feature = "IdbTransaction",
            feature = "IdbTransactionMode",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transaction_with_str_and_mode_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                store_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <IdbTransactionMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transaction_with_str_and_mode_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            store_names: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <IdbTransactionMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(store_names);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let store_names =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(store_names);
                let mode =
                    <IdbTransactionMode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_transaction_with_str_and_mode_IDBDatabase(self_, store_names, mode)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbDatabase",
    feature = "IdbTransaction",
    feature = "IdbTransactionMode",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_transaction_with_str_sequence_and_mode_IDBDatabase()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbTransactionMode as WasmDescribe>::describe();
    <IdbTransaction as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(
        feature = "IdbDatabase",
        feature = "IdbTransaction",
        feature = "IdbTransactionMode",
    ))]
    #[allow(bad_style)]
    #[doc = "The `transaction()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/transaction)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `IdbTransaction`, `IdbTransactionMode`*"]
    #[allow(clippy::all)]
    pub fn transaction_with_str_sequence_and_mode(
        &self,
        store_names: &::wasm_bindgen::JsValue,
        mode: IdbTransactionMode,
    ) -> Result<IdbTransaction, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbDatabase",
            feature = "IdbTransaction",
            feature = "IdbTransactionMode",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_transaction_with_str_sequence_and_mode_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                store_names: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <IdbTransactionMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_transaction_with_str_sequence_and_mode_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            store_names: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <IdbTransactionMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(store_names);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let store_names =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        store_names,
                    );
                let mode =
                    <IdbTransactionMode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_transaction_with_str_sequence_and_mode_IDBDatabase(
                    self_,
                    store_names,
                    mode,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbTransaction as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/name)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_IDBDatabase(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_version_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `version` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/version)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn version(&self) -> f64 {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_version_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_version_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_version_IDBDatabase(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DomStringList", feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_object_store_names_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <DomStringList as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "DomStringList", feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `objectStoreNames` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/objectStoreNames)\n\n*This API requires the following crate features to be activated: `DomStringList`, `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn object_store_names(&self) -> DomStringList {
        #[cfg(all(feature = "DomStringList", feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_object_store_names_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_object_store_names_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DomStringList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_object_store_names_IDBDatabase(self_)
            };
            <DomStringList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_IDBDatabase(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onabort)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_IDBDatabase(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_IDBDatabase(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onclose)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onclose);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_IDBDatabase(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_IDBDatabase(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onerror)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_IDBDatabase(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onversionchange_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onversionchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn onversionchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onversionchange_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onversionchange_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onversionchange_IDBDatabase(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbDatabase",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onversionchange_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase",))]
    #[allow(bad_style)]
    #[doc = "The `onversionchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/onversionchange)\n\n*This API requires the following crate features to be activated: `IdbDatabase`*"]
    #[allow(clippy::all)]
    pub fn set_onversionchange(&self, onversionchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbDatabase",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onversionchange_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onversionchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onversionchange_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onversionchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onversionchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onversionchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onversionchange,
                    );
                __widl_f_set_onversionchange_IDBDatabase(self_, onversionchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbDatabase", feature = "StorageType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_storage_IDBDatabase() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbDatabase as WasmDescribe>::describe();
    <StorageType as WasmDescribe>::describe();
}
impl IdbDatabase {
    #[cfg(all(feature = "IdbDatabase", feature = "StorageType",))]
    #[allow(bad_style)]
    #[doc = "The `storage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBDatabase/storage)\n\n*This API requires the following crate features to be activated: `IdbDatabase`, `StorageType`*"]
    #[allow(clippy::all)]
    pub fn storage(&self) -> StorageType {
        #[cfg(all(feature = "IdbDatabase", feature = "StorageType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_storage_IDBDatabase(
                self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StorageType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_storage_IDBDatabase(
            self_: <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StorageType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbDatabase as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_storage_IDBDatabase(self_)
            };
            <StorageType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_6f3800ee8a2d22de: [u8; 2199usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}U\x08\0\0\0\0\x17\0\0\x02\x0BIDBDatabase\x1D__widl_instanceof_IDBDatabase\0\0\0\0\x1A__widl_f_close_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x01\x05self_\x05close\0\0\0(__widl_f_create_mutable_file_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x02\x05self_\x04name\x11createMutableFile\0\0\02__widl_f_create_mutable_file_with_type_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x03\x05self_\x04name\x05type_\x11createMutableFile\0\0\0(__widl_f_create_object_store_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x02\x05self_\x04name\x11createObjectStore\0\0\0A__widl_f_create_object_store_with_optional_parameters_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x03\x05self_\x04name\x13optional_parameters\x11createObjectStore\0\0\0(__widl_f_delete_object_store_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x02\x05self_\x04name\x11deleteObjectStore\0\0\0)__widl_f_transaction_with_str_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x02\x05self_\x0Bstore_names\x0Btransaction\0\0\02__widl_f_transaction_with_str_sequence_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x02\x05self_\x0Bstore_names\x0Btransaction\0\0\02__widl_f_transaction_with_str_and_mode_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x03\x05self_\x0Bstore_names\x04mode\x0Btransaction\0\0\0;__widl_f_transaction_with_str_sequence_and_mode_IDBDatabase\x01\0\0\x01\x0BIDBDatabase\x01\0\0\x01\x03\x05self_\x0Bstore_names\x04mode\x0Btransaction\0\0\0\x19__widl_f_name_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0\x1C__widl_f_version_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x07version\x01\x01\x05self_\x07version\0\0\0'__widl_f_object_store_names_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x10objectStoreNames\x01\x01\x05self_\x10objectStoreNames\0\0\0\x1C__widl_f_onabort_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0 __widl_f_set_onabort_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x1C__widl_f_onclose_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0 __widl_f_set_onclose_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0\x1C__widl_f_onerror_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0 __widl_f_set_onerror_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0$__widl_f_onversionchange_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x0Fonversionchange\x01\x01\x05self_\x0Fonversionchange\0\0\0(__widl_f_set_onversionchange_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x02\x0Fonversionchange\x01\x02\x05self_\x0Fonversionchange\x0Fonversionchange\0\0\0\x1C__widl_f_storage_IDBDatabase\0\0\0\x01\x0BIDBDatabase\x01\0\x01\x07storage\x01\x01\x05self_\x07storage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
