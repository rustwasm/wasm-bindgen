use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `SourceBuffer` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct SourceBuffer {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_SourceBuffer: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for SourceBuffer {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(83u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
            inform(66u32);
            inform(117u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for SourceBuffer {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for SourceBuffer {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for SourceBuffer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a SourceBuffer {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for SourceBuffer {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            SourceBuffer {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for SourceBuffer {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a SourceBuffer {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for SourceBuffer {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<SourceBuffer>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(SourceBuffer {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for SourceBuffer {
        #[inline]
        fn from(obj: JsValue) -> SourceBuffer {
            SourceBuffer { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for SourceBuffer {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<SourceBuffer> for SourceBuffer {
        #[inline]
        fn as_ref(&self) -> &SourceBuffer {
            self
        }
    }
    impl From<SourceBuffer> for JsValue {
        #[inline]
        fn from(obj: SourceBuffer) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for SourceBuffer {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_SourceBuffer(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_SourceBuffer(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_SourceBuffer(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            SourceBuffer { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const SourceBuffer) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<SourceBuffer> for EventTarget {
    #[inline]
    fn from(obj: SourceBuffer) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for SourceBuffer {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<SourceBuffer> for ::js_sys::Object {
    #[inline]
    fn from(obj: SourceBuffer) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for SourceBuffer {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_abort_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `abort()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/abort)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn abort(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_abort_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_abort_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_abort_SourceBuffer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_buffer_with_array_buffer_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_buffer_with_array_buffer(
        &self,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_buffer_with_array_buffer_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_buffer_with_array_buffer_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_buffer_with_array_buffer_SourceBuffer(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_buffer_with_array_buffer_view_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_buffer_with_array_buffer_view(
        &self,
        data: &::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_buffer_with_array_buffer_view_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_buffer_with_array_buffer_view_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_buffer_with_array_buffer_view_SourceBuffer(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_buffer_with_u8_array_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBuffer)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_buffer_with_u8_array(
        &self,
        data: &mut [u8],
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_buffer_with_u8_array_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_buffer_with_u8_array_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_buffer_with_u8_array_SourceBuffer(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_buffer_async_with_array_buffer_SourceBuffer()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendBufferAsync()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_buffer_async_with_array_buffer(
        &self,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_buffer_async_with_array_buffer_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_buffer_async_with_array_buffer_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_buffer_async_with_array_buffer_SourceBuffer(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_buffer_async_with_array_buffer_view_SourceBuffer(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendBufferAsync()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_buffer_async_with_array_buffer_view(
        &self,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_buffer_async_with_array_buffer_view_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_buffer_async_with_array_buffer_view_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_buffer_async_with_array_buffer_view_SourceBuffer(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_buffer_async_with_u8_array_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendBufferAsync()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendBufferAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_buffer_async_with_u8_array(
        &self,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_buffer_async_with_u8_array_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_buffer_async_with_u8_array_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_append_buffer_async_with_u8_array_SourceBuffer(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_change_type_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `changeType()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/changeType)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn change_type(&self, type_: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_change_type_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_change_type_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_change_type_SourceBuffer(self_, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/remove)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn remove(&self, start: f64, end: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_remove_SourceBuffer(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_async_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `removeAsync()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/removeAsync)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn remove_async(
        &self,
        start: f64,
        end: f64,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_async_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_async_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_remove_async_SourceBuffer(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SourceBuffer", feature = "SourceBufferAppendMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mode_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <SourceBufferAppendMode as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer", feature = "SourceBufferAppendMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
    #[allow(clippy::all)]
    pub fn mode(&self) -> SourceBufferAppendMode {
        #[cfg(all(feature = "SourceBuffer", feature = "SourceBufferAppendMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mode_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SourceBufferAppendMode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mode_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SourceBufferAppendMode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mode_SourceBuffer(self_)
            };
            <SourceBufferAppendMode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer", feature = "SourceBufferAppendMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_mode_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <SourceBufferAppendMode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer", feature = "SourceBufferAppendMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/mode)\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferAppendMode`*"]
    #[allow(clippy::all)]
    pub fn set_mode(&self, mode: SourceBufferAppendMode) {
        #[cfg(all(feature = "SourceBuffer", feature = "SourceBufferAppendMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_mode_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <SourceBufferAppendMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_mode_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <SourceBufferAppendMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode =
                    <SourceBufferAppendMode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_set_mode_SourceBuffer(self_, mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_updating_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `updating` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/updating)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn updating(&self) -> bool {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_updating_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_updating_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_updating_SourceBuffer(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer", feature = "TimeRanges",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffered_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <TimeRanges as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer", feature = "TimeRanges",))]
    #[allow(bad_style)]
    #[doc = "The `buffered` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/buffered)\n\n*This API requires the following crate features to be activated: `SourceBuffer`, `TimeRanges`*"]
    #[allow(clippy::all)]
    pub fn buffered(&self) -> Result<TimeRanges, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "SourceBuffer", feature = "TimeRanges",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffered_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffered_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TimeRanges as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffered_SourceBuffer(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<TimeRanges as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timestamp_offset_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `timestampOffset` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn timestamp_offset(&self) -> f64 {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timestamp_offset_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timestamp_offset_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timestamp_offset_SourceBuffer(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timestamp_offset_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `timestampOffset` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/timestampOffset)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_timestamp_offset(&self, timestamp_offset: f64) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timestamp_offset_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timestamp_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timestamp_offset_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timestamp_offset: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(timestamp_offset);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let timestamp_offset =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(timestamp_offset);
                __widl_f_set_timestamp_offset_SourceBuffer(self_, timestamp_offset)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_window_start_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendWindowStart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_window_start(&self) -> f64 {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_window_start_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_window_start_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_window_start_SourceBuffer(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_append_window_start_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendWindowStart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowStart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_append_window_start(&self, append_window_start: f64) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_append_window_start_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                append_window_start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_append_window_start_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            append_window_start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(append_window_start);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let append_window_start =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(append_window_start);
                __widl_f_set_append_window_start_SourceBuffer(self_, append_window_start)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_append_window_end_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendWindowEnd` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn append_window_end(&self) -> f64 {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_append_window_end_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_append_window_end_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_append_window_end_SourceBuffer(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_append_window_end_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `appendWindowEnd` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/appendWindowEnd)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_append_window_end(&self, append_window_end: f64) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_append_window_end_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                append_window_end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_append_window_end_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            append_window_end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(append_window_end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let append_window_end =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(append_window_end);
                __widl_f_set_append_window_end_SourceBuffer(self_, append_window_end)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onupdatestart_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onupdatestart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn onupdatestart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onupdatestart_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onupdatestart_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onupdatestart_SourceBuffer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onupdatestart_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onupdatestart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdatestart)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_onupdatestart(&self, onupdatestart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onupdatestart_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onupdatestart : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onupdatestart_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onupdatestart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onupdatestart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onupdatestart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onupdatestart,
                    );
                __widl_f_set_onupdatestart_SourceBuffer(self_, onupdatestart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onupdate_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onupdate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn onupdate(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onupdate_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onupdate_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onupdate_SourceBuffer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onupdate_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onupdate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdate)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_onupdate(&self, onupdate: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onupdate_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onupdate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onupdate_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onupdate: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onupdate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onupdate =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onupdate,
                    );
                __widl_f_set_onupdate_SourceBuffer(self_, onupdate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onupdateend_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onupdateend` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn onupdateend(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onupdateend_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onupdateend_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onupdateend_SourceBuffer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onupdateend_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onupdateend` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onupdateend)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_onupdateend(&self, onupdateend: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onupdateend_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onupdateend : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onupdateend_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onupdateend: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onupdateend);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onupdateend =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onupdateend,
                    );
                __widl_f_set_onupdateend_SourceBuffer(self_, onupdateend)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_SourceBuffer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onerror)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_SourceBuffer(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onabort_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn onabort(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onabort_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onabort_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onabort_SourceBuffer(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onabort_SourceBuffer() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&SourceBuffer as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl SourceBuffer {
    #[cfg(all(feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `onabort` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBuffer/onabort)\n\n*This API requires the following crate features to be activated: `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn set_onabort(&self, onabort: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onabort_SourceBuffer(
                self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onabort: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onabort_SourceBuffer(
            self_: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onabort =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onabort,
                    );
                __widl_f_set_onabort_SourceBuffer(self_, onabort)
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
pub static __WASM_BINDGEN_GENERATED_fba00449160a6897: [u8; 3025usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8F\x0B\0\0\0\0\x1F\0\0\x02\x0CSourceBuffer\x1E__widl_instanceof_SourceBuffer\0\0\0\0\x1B__widl_f_abort_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x01\x05self_\x05abort\0\0\05__widl_f_append_buffer_with_array_buffer_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x04data\x0CappendBuffer\0\0\0:__widl_f_append_buffer_with_array_buffer_view_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x04data\x0CappendBuffer\0\0\01__widl_f_append_buffer_with_u8_array_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x04data\x0CappendBuffer\0\0\0;__widl_f_append_buffer_async_with_array_buffer_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x04data\x11appendBufferAsync\0\0\0@__widl_f_append_buffer_async_with_array_buffer_view_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x04data\x11appendBufferAsync\0\0\07__widl_f_append_buffer_async_with_u8_array_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x04data\x11appendBufferAsync\0\0\0!__widl_f_change_type_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x02\x05self_\x05type_\nchangeType\0\0\0\x1C__widl_f_remove_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x03\x05self_\x05start\x03end\x06remove\0\0\0\"__widl_f_remove_async_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\0\x01\x03\x05self_\x05start\x03end\x0BremoveAsync\0\0\0\x1A__widl_f_mode_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x04mode\x01\x01\x05self_\x04mode\0\0\0\x1E__widl_f_set_mode_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x04mode\x01\x02\x05self_\x04mode\x04mode\0\0\0\x1E__widl_f_updating_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x08updating\x01\x01\x05self_\x08updating\0\0\0\x1E__widl_f_buffered_SourceBuffer\x01\0\0\x01\x0CSourceBuffer\x01\0\x01\x08buffered\x01\x01\x05self_\x08buffered\0\0\0&__widl_f_timestamp_offset_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x0FtimestampOffset\x01\x01\x05self_\x0FtimestampOffset\0\0\0*__widl_f_set_timestamp_offset_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x0FtimestampOffset\x01\x02\x05self_\x10timestamp_offset\x0FtimestampOffset\0\0\0)__widl_f_append_window_start_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x11appendWindowStart\x01\x01\x05self_\x11appendWindowStart\0\0\0-__widl_f_set_append_window_start_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x11appendWindowStart\x01\x02\x05self_\x13append_window_start\x11appendWindowStart\0\0\0'__widl_f_append_window_end_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x0FappendWindowEnd\x01\x01\x05self_\x0FappendWindowEnd\0\0\0+__widl_f_set_append_window_end_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x0FappendWindowEnd\x01\x02\x05self_\x11append_window_end\x0FappendWindowEnd\0\0\0#__widl_f_onupdatestart_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\ronupdatestart\x01\x01\x05self_\ronupdatestart\0\0\0'__widl_f_set_onupdatestart_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\ronupdatestart\x01\x02\x05self_\ronupdatestart\ronupdatestart\0\0\0\x1E__widl_f_onupdate_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x08onupdate\x01\x01\x05self_\x08onupdate\0\0\0\"__widl_f_set_onupdate_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x08onupdate\x01\x02\x05self_\x08onupdate\x08onupdate\0\0\0!__widl_f_onupdateend_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x0Bonupdateend\x01\x01\x05self_\x0Bonupdateend\0\0\0%__widl_f_set_onupdateend_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x0Bonupdateend\x01\x02\x05self_\x0Bonupdateend\x0Bonupdateend\0\0\0\x1D__widl_f_onerror_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0!__widl_f_set_onerror_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1D__widl_f_onabort_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x01\x07onabort\x01\x01\x05self_\x07onabort\0\0\0!__widl_f_set_onabort_SourceBuffer\0\0\0\x01\x0CSourceBuffer\x01\0\x02\x07onabort\x01\x02\x05self_\x07onabort\x07onabort\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
