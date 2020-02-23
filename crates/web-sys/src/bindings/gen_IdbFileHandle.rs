use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `IDBFileHandle` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct IdbFileHandle {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_IdbFileHandle: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for IdbFileHandle {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(73u32);
            inform(68u32);
            inform(66u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(101u32);
            inform(72u32);
            inform(97u32);
            inform(110u32);
            inform(100u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for IdbFileHandle {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for IdbFileHandle {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for IdbFileHandle {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a IdbFileHandle {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for IdbFileHandle {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            IdbFileHandle {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for IdbFileHandle {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a IdbFileHandle {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for IdbFileHandle {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<IdbFileHandle>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(IdbFileHandle {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for IdbFileHandle {
        #[inline]
        fn from(obj: JsValue) -> IdbFileHandle {
            IdbFileHandle { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for IdbFileHandle {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<IdbFileHandle> for IdbFileHandle {
        #[inline]
        fn as_ref(&self) -> &IdbFileHandle {
            self
        }
    }
    impl From<IdbFileHandle> for JsValue {
        #[inline]
        fn from(obj: IdbFileHandle) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for IdbFileHandle {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_IDBFileHandle(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_IDBFileHandle(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_IDBFileHandle(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            IdbFileHandle { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const IdbFileHandle) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<IdbFileHandle> for EventTarget {
    #[inline]
    fn from(obj: IdbFileHandle) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for IdbFileHandle {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<IdbFileHandle> for ::js_sys::Object {
    #[inline]
    fn from(obj: IdbFileHandle) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for IdbFileHandle {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/abort)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_IDBFileHandle(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_str_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn append_with_str(
        &self,
        value: &str,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_str_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_str_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_append_with_str_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_array_buffer_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn append_with_array_buffer(
        &self,
        value: &::js_sys::ArrayBuffer,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_array_buffer_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_array_buffer_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_append_with_array_buffer_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_array_buffer_view_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn append_with_array_buffer_view(
        &self,
        value: &::js_sys::Object,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_array_buffer_view_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_array_buffer_view_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_append_with_array_buffer_view_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_u8_array_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn append_with_u8_array(
        &self,
        value: &mut [u8],
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_u8_array_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_u8_array_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_append_with_u8_array_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Blob",
    feature = "IdbFileHandle",
    feature = "IdbFileRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_with_blob_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(
        feature = "Blob",
        feature = "IdbFileHandle",
        feature = "IdbFileRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `append()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/append)\n\n*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn append_with_blob(
        &self,
        value: &Blob,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Blob",
            feature = "IdbFileHandle",
            feature = "IdbFileRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_with_blob_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_with_blob_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_append_with_blob_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_flush_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `flush()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/flush)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn flush(&self) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_flush_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_flush_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_flush_IDBFileHandle(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_metadata_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getMetadata()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn get_metadata(&self) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_metadata_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_metadata_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_metadata_IDBFileHandle(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "IdbFileHandle",
    feature = "IdbFileMetadataParameters",
    feature = "IdbFileRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_metadata_with_parameters_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&IdbFileMetadataParameters as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(
        feature = "IdbFileHandle",
        feature = "IdbFileMetadataParameters",
        feature = "IdbFileRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `getMetadata()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/getMetadata)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileMetadataParameters`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn get_metadata_with_parameters(
        &self,
        parameters: &IdbFileMetadataParameters,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "IdbFileHandle",
            feature = "IdbFileMetadataParameters",
            feature = "IdbFileRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_metadata_with_parameters_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                parameters: <&IdbFileMetadataParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_metadata_with_parameters_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            parameters: <&IdbFileMetadataParameters as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(parameters);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let parameters =
                    <&IdbFileMetadataParameters as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        parameters,
                    );
                __widl_f_get_metadata_with_parameters_IDBFileHandle(self_, parameters)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_array_buffer_with_u32_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readAsArrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn read_as_array_buffer_with_u32(
        &self,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_array_buffer_with_u32_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_array_buffer_with_u32_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_read_as_array_buffer_with_u32_IDBFileHandle(self_, size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_array_buffer_with_f64_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readAsArrayBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsArrayBuffer)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn read_as_array_buffer_with_f64(
        &self,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_array_buffer_with_f64_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_array_buffer_with_f64_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_read_as_array_buffer_with_f64_IDBFileHandle(self_, size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_with_u32_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn read_as_text_with_u32(
        &self,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_with_u32_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_with_u32_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_read_as_text_with_u32_IDBFileHandle(self_, size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_with_f64_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn read_as_text_with_f64(
        &self,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_with_f64_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_with_f64_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_read_as_text_with_f64_IDBFileHandle(self_, size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_with_u32_and_encoding_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn read_as_text_with_u32_and_encoding(
        &self,
        size: u32,
        encoding: Option<&str>,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_with_u32_and_encoding_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoding: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_with_u32_and_encoding_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoding: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            drop(encoding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                let encoding =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(encoding);
                __widl_f_read_as_text_with_u32_and_encoding_IDBFileHandle(self_, size, encoding)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_read_as_text_with_f64_and_encoding_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Option<&str> as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `readAsText()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/readAsText)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn read_as_text_with_f64_and_encoding(
        &self,
        size: f64,
        encoding: Option<&str>,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_read_as_text_with_f64_and_encoding_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                encoding: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_read_as_text_with_f64_and_encoding_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            encoding: <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            drop(encoding);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                let encoding =
                    <Option<&str> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(encoding);
                __widl_f_read_as_text_with_f64_and_encoding_IDBFileHandle(self_, size, encoding)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_truncate_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `truncate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn truncate(&self) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_truncate_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_truncate_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_truncate_IDBFileHandle(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_truncate_with_u32_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `truncate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn truncate_with_u32(
        &self,
        size: u32,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_truncate_with_u32_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_truncate_with_u32_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_truncate_with_u32_IDBFileHandle(self_, size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_truncate_with_f64_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `truncate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/truncate)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn truncate_with_f64(
        &self,
        size: f64,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_truncate_with_f64_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_truncate_with_f64_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            size: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let size = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(size);
                __widl_f_truncate_with_f64_IDBFileHandle(self_, size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_with_str_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn write_with_str(
        &self,
        value: &str,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_with_str_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_with_str_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_write_with_str_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_with_array_buffer_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn write_with_array_buffer(
        &self,
        value: &::js_sys::ArrayBuffer,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_with_array_buffer_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_with_array_buffer_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_write_with_array_buffer_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_with_array_buffer_view_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn write_with_array_buffer_view(
        &self,
        value: &::js_sys::Object,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_with_array_buffer_view_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_with_array_buffer_view_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_write_with_array_buffer_view_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_with_u8_array_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn write_with_u8_array(
        &self,
        value: &mut [u8],
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbFileRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_with_u8_array_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_with_u8_array_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_write_with_u8_array_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Blob",
    feature = "IdbFileHandle",
    feature = "IdbFileRequest",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_write_with_blob_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <Option<IdbFileRequest> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(
        feature = "Blob",
        feature = "IdbFileHandle",
        feature = "IdbFileRequest",
    ))]
    #[allow(bad_style)]
    #[doc = "The `write()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/write)\n\n*This API requires the following crate features to be activated: `Blob`, `IdbFileHandle`, `IdbFileRequest`*"]
    #[allow(clippy::all)]
    pub fn write_with_blob(
        &self,
        value: &Blob,
    ) -> Result<Option<IdbFileRequest>, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Blob",
            feature = "IdbFileHandle",
            feature = "IdbFileRequest",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_write_with_blob_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                value: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_write_with_blob_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            value: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(value);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let value = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(value);
                __widl_f_write_with_blob_IDBFileHandle(self_, value)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Option<IdbFileRequest> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mutable_file_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<IdbMutableFile> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `mutableFile` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/mutableFile)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn mutable_file(&self) -> Option<IdbMutableFile> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mutable_file_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbMutableFile> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mutable_file_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbMutableFile> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mutable_file_IDBFileHandle(self_)
            };
            <Option<IdbMutableFile> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_file_handle_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<IdbMutableFile> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
    #[allow(bad_style)]
    #[doc = "The `fileHandle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/fileHandle)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[allow(clippy::all)]
    pub fn file_handle(&self) -> Option<IdbMutableFile> {
        #[cfg(all(feature = "IdbFileHandle", feature = "IdbMutableFile",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_file_handle_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<IdbMutableFile> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_file_handle_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<IdbMutableFile> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_file_handle_IDBFileHandle(self_)
            };
            <Option<IdbMutableFile> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `active` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/active)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn active(&self) -> bool {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_IDBFileHandle(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_location_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `location` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn location(&self) -> Option<f64> {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_location_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_location_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_location_IDBFileHandle(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_location_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `location` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/location)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn set_location(&self, location: Option<f64>) {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_location_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                location: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_location_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            location: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(location);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let location =
                    <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(location);
                __widl_f_set_location_IDBFileHandle(self_, location)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncomplete_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `oncomplete` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn oncomplete(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncomplete_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncomplete_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncomplete_IDBFileHandle(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncomplete_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `oncomplete` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/oncomplete)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn set_oncomplete(&self, oncomplete: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncomplete_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncomplete : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncomplete_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncomplete =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncomplete,
                    );
                __widl_f_set_oncomplete_IDBFileHandle(self_, oncomplete)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_IDBFileHandle(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onabort)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_IDBFileHandle(self_, onabort)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_IDBFileHandle(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IdbFileHandle",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_IDBFileHandle() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&IdbFileHandle as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl IdbFileHandle {
    #[cfg(all(feature = "IdbFileHandle",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBFileHandle/onerror)\n\n*This API requires the following crate features to be activated: `IdbFileHandle`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "IdbFileHandle",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_IDBFileHandle(
                self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_IDBFileHandle(
            self_: <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&IdbFileHandle as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_IDBFileHandle(self_, onerror)
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
pub static __WASM_BINDGEN_GENERATED_fb00ead4be8d94bc: [u8; 3208usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}F\x0C\0\0\0\0#\0\0\x02\rIDBFileHandle\x1F__widl_instanceof_IDBFileHandle\0\0\0\0\x1C__widl_f_abort_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x01\x05self_\x05abort\0\0\0&__widl_f_append_with_str_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x06append\0\0\0/__widl_f_append_with_array_buffer_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x06append\0\0\04__widl_f_append_with_array_buffer_view_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x06append\0\0\0+__widl_f_append_with_u8_array_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x06append\0\0\0'__widl_f_append_with_blob_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x06append\0\0\0\x1C__widl_f_flush_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x01\x05self_\x05flush\0\0\0#__widl_f_get_metadata_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x01\x05self_\x0BgetMetadata\0\0\03__widl_f_get_metadata_with_parameters_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\nparameters\x0BgetMetadata\0\0\04__widl_f_read_as_array_buffer_with_u32_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x04size\x11readAsArrayBuffer\0\0\04__widl_f_read_as_array_buffer_with_f64_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x04size\x11readAsArrayBuffer\0\0\0,__widl_f_read_as_text_with_u32_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x04size\nreadAsText\0\0\0,__widl_f_read_as_text_with_f64_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x04size\nreadAsText\0\0\09__widl_f_read_as_text_with_u32_and_encoding_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x03\x05self_\x04size\x08encoding\nreadAsText\0\0\09__widl_f_read_as_text_with_f64_and_encoding_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x03\x05self_\x04size\x08encoding\nreadAsText\0\0\0\x1F__widl_f_truncate_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x01\x05self_\x08truncate\0\0\0(__widl_f_truncate_with_u32_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x04size\x08truncate\0\0\0(__widl_f_truncate_with_f64_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x04size\x08truncate\0\0\0%__widl_f_write_with_str_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x05write\0\0\0.__widl_f_write_with_array_buffer_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x05write\0\0\03__widl_f_write_with_array_buffer_view_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x05write\0\0\0*__widl_f_write_with_u8_array_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x05write\0\0\0&__widl_f_write_with_blob_IDBFileHandle\x01\0\0\x01\rIDBFileHandle\x01\0\0\x01\x02\x05self_\x05value\x05write\0\0\0#__widl_f_mutable_file_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\x0BmutableFile\x01\x01\x05self_\x0BmutableFile\0\0\0\"__widl_f_file_handle_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\nfileHandle\x01\x01\x05self_\nfileHandle\0\0\0\x1D__widl_f_active_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\x06active\x01\x01\x05self_\x06active\0\0\0\x1F__widl_f_location_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\x08location\x01\x01\x05self_\x08location\0\0\0#__widl_f_set_location_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x02\x08location\x01\x02\x05self_\x08location\x08location\0\0\0!__widl_f_oncomplete_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\noncomplete\x01\x01\x05self_\noncomplete\0\0\0%__widl_f_set_oncomplete_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x02\noncomplete\x01\x02\x05self_\noncomplete\noncomplete\0\0\0\x1E__widl_f_onabort_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0\"__widl_f_set_onabort_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\x1E__widl_f_onerror_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\"__widl_f_set_onerror_IDBFileHandle\0\0\0\x01\rIDBFileHandle\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
