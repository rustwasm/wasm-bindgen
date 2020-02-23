use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBCursor` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbCursor {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbCursor: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbCursor {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(67u32);
            inform(117u32);
            inform(114u32);
            inform(115u32);
            inform(111u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for IdbCursor {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbCursor {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbCursor {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbCursor {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbCursor {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbCursor {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbCursor {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbCursor {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbCursor {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbCursor>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbCursor {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbCursor {
        #[inline]
        fn from(obj: JsValue) -> IdbCursor {
            IdbCursor { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbCursor {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbCursor> for IdbCursor {
        #[inline]
        fn as_ref(&self) -> &IdbCursor {
            self
        }
    }
    impl From<IdbCursor> for JsValue {
        #[inline]
        fn from(obj: IdbCursor) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbCursor {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBCursor(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBCursor(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBCursor(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbCursor { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbCursor) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbCursor> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbCursor) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbCursor {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_advance_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbCursor as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `advance()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/advance)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn advance(&self, count: u32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_advance_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_advance_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            count: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(count);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let count = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(count);
                __widl_f_advance_IDBCursor(self_, count)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_continue_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursor as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `continue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn continue_(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_continue_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_continue_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_continue_IDBCursor(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_continue_with_key_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbCursor as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `continue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continue)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn continue_with_key(
        &self,
        key: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_continue_with_key_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_continue_with_key_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                __widl_f_continue_with_key_IDBCursor(self_, key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_continue_primary_key_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbCursor as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `continuePrimaryKey()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/continuePrimaryKey)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn continue_primary_key(
        &self,
        key: &::wasm_bindgen::JsValue,
        primary_key: &::wasm_bindgen::JsValue,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_continue_primary_key_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                primary_key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_continue_primary_key_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            primary_key: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(key);
            drop(primary_key);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(key);
                let primary_key =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        primary_key,
                    );
                __widl_f_continue_primary_key_IDBCursor(self_, key, primary_key)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbCursor", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_delete_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursor as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `delete()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/delete)\n\n*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn delete(&self) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_delete_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_delete_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_delete_IDBCursor(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbCursor", feature = "IdbRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbCursor as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IdbRequest as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor", feature = "IdbRequest",))]
    #[allow(bad_style)]
    #[doc = "The `update()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/update)\n\n*This API requires the following crate features to be activated: `IdbCursor`, `IdbRequest`*"]
    #[allow(clippy::all)]
    pub fn update(
        &self,
        value: &::wasm_bindgen::JsValue,
    ) -> Result<IdbRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor", feature = "IdbRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        value,
                    );
                __widl_f_update_IDBCursor(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IdbRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursor as WasmDescribe>::describe();
    <::js_sys::Object as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `source` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/source)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn source(&self) -> ::js_sys::Object {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_source_IDBCursor(self_)
            };
            <::js_sys::Object as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbCursor", feature = "IdbCursorDirection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_direction_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursor as WasmDescribe>::describe();
    <IdbCursorDirection as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor", feature = "IdbCursorDirection",))]
    #[allow(bad_style)]
    #[doc = "The `direction` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/direction)\n\n*This API requires the following crate features to be activated: `IdbCursor`, `IdbCursorDirection`*"]
    #[allow(clippy::all)]
    pub fn direction(&self) -> IdbCursorDirection {
        #[cfg(all(feature = "IdbCursor", feature = "IdbCursorDirection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_direction_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IdbCursorDirection as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_direction_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IdbCursorDirection as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_direction_IDBCursor(self_)
            };
            <IdbCursorDirection as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursor as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `key` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/key)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn key(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_IDBCursor(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbCursor",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_primary_key_IDBCursor() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbCursor as WasmDescribe>::describe();
    <::wasm_bindgen::JsValue as WasmDescribe>::describe();
}
impl IdbCursor {
    #[cfg(all(feature = "IdbCursor",))]
    #[allow(bad_style)]
    #[doc = "The `primaryKey` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBCursor/primaryKey)\n\n*This API requires the following crate features to be activated: `IdbCursor`*"]
    #[allow(clippy::all)]
    pub fn primary_key(&self) -> Result<::wasm_bindgen::JsValue, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbCursor",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_primary_key_IDBCursor(
                self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_primary_key_IDBCursor(
            self_: <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbCursor as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_primary_key_IDBCursor(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_036105a7c297a8c4: [u8; 875usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"})\x03\0\0\0\0\x0B\0\0\x02\tIDBCursor\x1B__widl_instanceof_IDBCursor\0\0\0\0\x1A__widl_f_advance_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\0\x01\x02\x05self_\x05count\x07advance\0\0\0\x1B__widl_f_continue_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\0\x01\x01\x05self_\x08continue\0\0\0$__widl_f_continue_with_key_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\0\x01\x02\x05self_\x03key\x08continue\0\0\0'__widl_f_continue_primary_key_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\0\x01\x03\x05self_\x03key\x0Bprimary_key\x12continuePrimaryKey\0\0\0\x19__widl_f_delete_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\0\x01\x01\x05self_\x06delete\0\0\0\x19__widl_f_update_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\0\x01\x02\x05self_\x05value\x06update\0\0\0\x19__widl_f_source_IDBCursor\0\0\0\x01\tIDBCursor\x01\0\x01\x06source\x01\x01\x05self_\x06source\0\0\0\x1C__widl_f_direction_IDBCursor\0\0\0\x01\tIDBCursor\x01\0\x01\tdirection\x01\x01\x05self_\tdirection\0\0\0\x16__widl_f_key_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\x01\x03key\x01\x01\x05self_\x03key\0\0\0\x1E__widl_f_primary_key_IDBCursor\x01\0\0\x01\tIDBCursor\x01\0\x01\nprimaryKey\x01\x01\x05self_\nprimaryKey\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
