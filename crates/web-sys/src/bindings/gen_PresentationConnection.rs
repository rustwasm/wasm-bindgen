use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PresentationConnection` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PresentationConnection {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PresentationConnection: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PresentationConnection {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(22u32);
            inform(80u32);
            inform(114u32);
            inform(101u32);
            inform(115u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for PresentationConnection {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for PresentationConnection {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PresentationConnection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PresentationConnection {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PresentationConnection {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PresentationConnection {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PresentationConnection {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PresentationConnection {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PresentationConnection {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PresentationConnection>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PresentationConnection {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PresentationConnection {
        #[inline]
        fn from(obj: JsValue) -> PresentationConnection {
            PresentationConnection { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PresentationConnection {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PresentationConnection> for PresentationConnection {
        #[inline]
        fn as_ref(&self) -> &PresentationConnection {
            self
        }
    }
    impl From<PresentationConnection> for JsValue {
        #[inline]
        fn from(obj: PresentationConnection) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PresentationConnection {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PresentationConnection(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PresentationConnection(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PresentationConnection(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PresentationConnection { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PresentationConnection) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PresentationConnection> for EventTarget {
    #[inline]
    fn from(obj: PresentationConnection) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for PresentationConnection {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PresentationConnection> for ::js_sys::Object {
    #[inline]
    fn from(obj: PresentationConnection) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PresentationConnection {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/close)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn close(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_PresentationConnection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_str_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn send_with_str(&self, data: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_str_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_str_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_str_PresentationConnection(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_blob_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "Blob", feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `Blob`, `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn send_with_blob(&self, data: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_blob_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_blob_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_blob_PresentationConnection(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer(
        &self,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_PresentationConnection(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_view_PresentationConnection()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer_view(
        &self,
        data: &::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_view_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_view_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_view_PresentationConnection(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_u8_array_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn send_with_u8_array(&self, data: &mut [u8]) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_u8_array_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_u8_array_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_u8_array_PresentationConnection(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_terminate_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `terminate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/terminate)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn terminate(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_terminate_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_terminate_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_terminate_PresentationConnection(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/id)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_PresentationConnection(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_url_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `url` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/url)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn url(&self) -> String {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_url_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_url_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_url_PresentationConnection(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "PresentationConnection",
    feature = "PresentationConnectionState",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <PresentationConnectionState as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(
        feature = "PresentationConnection",
        feature = "PresentationConnectionState",
    ))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/state)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionState`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> PresentationConnectionState {
        #[cfg(all(
            feature = "PresentationConnection",
            feature = "PresentationConnectionState",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PresentationConnectionState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PresentationConnectionState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_PresentationConnection(self_)
            };
            <PresentationConnectionState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onconnect_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onconnect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn onconnect(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onconnect_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onconnect_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onconnect_PresentationConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onconnect_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onconnect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onconnect(&self, onconnect: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onconnect_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onconnect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onconnect_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onconnect: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onconnect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onconnect =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onconnect,
                    );
                __widl_f_set_onconnect_PresentationConnection(self_, onconnect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_PresentationConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_PresentationConnection(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onterminate_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onterminate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn onterminate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onterminate_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onterminate_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onterminate_PresentationConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onterminate_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onterminate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onterminate(&self, onterminate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onterminate_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onterminate : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onterminate_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onterminate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onterminate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onterminate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onterminate,
                    );
                __widl_f_set_onterminate_PresentationConnection(self_, onterminate)
            };
            ()
        }
    }
}
#[cfg(all(
    feature = "PresentationConnection",
    feature = "PresentationConnectionBinaryType",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_binary_type_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <PresentationConnectionBinaryType as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(
        feature = "PresentationConnection",
        feature = "PresentationConnectionBinaryType",
    ))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
    #[allow(clippy::all)]
    pub fn binary_type(&self) -> PresentationConnectionBinaryType {
        #[cfg(all(
            feature = "PresentationConnection",
            feature = "PresentationConnectionBinaryType",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_binary_type_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PresentationConnectionBinaryType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_binary_type_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PresentationConnectionBinaryType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_binary_type_PresentationConnection(self_)
            };
            <PresentationConnectionBinaryType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(
    feature = "PresentationConnection",
    feature = "PresentationConnectionBinaryType",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_binary_type_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <PresentationConnectionBinaryType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(
        feature = "PresentationConnection",
        feature = "PresentationConnectionBinaryType",
    ))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)\n\n*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
    #[allow(clippy::all)]
    pub fn set_binary_type(&self, binary_type: PresentationConnectionBinaryType) {
        #[cfg(all(
            feature = "PresentationConnection",
            feature = "PresentationConnectionBinaryType",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_binary_type_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                binary_type : < PresentationConnectionBinaryType as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_binary_type_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            binary_type : < PresentationConnectionBinaryType as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(binary_type);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let binary_type = < PresentationConnectionBinaryType as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( binary_type ) ;
                __widl_f_set_binary_type_PresentationConnection(self_, binary_type)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_PresentationConnection(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationConnection",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_PresentationConnection() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationConnection as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationConnection {
    #[cfg(all(feature = "PresentationConnection",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)\n\n*This API requires the following crate features to be activated: `PresentationConnection`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "PresentationConnection",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_PresentationConnection(
                self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_PresentationConnection(
            self_: <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationConnection as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_PresentationConnection(self_, onmessage)
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
pub static __WASM_BINDGEN_GENERATED_655fec3f1553d77e: [u8; 2233usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}w\x08\0\0\0\0\x15\0\0\x02\x16PresentationConnection(__widl_instanceof_PresentationConnection\0\0\0\0%__widl_f_close_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x01\x05self_\x05close\0\0\0-__widl_f_send_with_str_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0.__widl_f_send_with_blob_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\06__widl_f_send_with_array_buffer_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0;__widl_f_send_with_array_buffer_view_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\02__widl_f_send_with_u8_array_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0)__widl_f_terminate_PresentationConnection\x01\0\0\x01\x16PresentationConnection\x01\0\0\x01\x01\x05self_\tterminate\0\0\0\"__widl_f_id_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0#__widl_f_url_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\x03url\x01\x01\x05self_\x03url\0\0\0%__widl_f_state_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0)__widl_f_onconnect_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\tonconnect\x01\x01\x05self_\tonconnect\0\0\0-__widl_f_set_onconnect_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x02\tonconnect\x01\x02\x05self_\tonconnect\tonconnect\0\0\0'__widl_f_onclose_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0+__widl_f_set_onclose_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0+__widl_f_onterminate_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\x0Bonterminate\x01\x01\x05self_\x0Bonterminate\0\0\0/__widl_f_set_onterminate_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x02\x0Bonterminate\x01\x02\x05self_\x0Bonterminate\x0Bonterminate\0\0\0+__widl_f_binary_type_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\nbinaryType\x01\x01\x05self_\nbinaryType\0\0\0/__widl_f_set_binary_type_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x02\nbinaryType\x01\x02\x05self_\x0Bbinary_type\nbinaryType\0\0\0)__widl_f_onmessage_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0-__widl_f_set_onmessage_PresentationConnection\0\0\0\x01\x16PresentationConnection\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
