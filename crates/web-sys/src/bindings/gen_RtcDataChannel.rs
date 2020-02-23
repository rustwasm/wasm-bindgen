use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `RTCDataChannel` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct RtcDataChannel {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_RtcDataChannel: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for RtcDataChannel {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(82u32);
            inform(84u32);
            inform(67u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
            inform(67u32);
            inform(104u32);
            inform(97u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(108u32);
        }
    }
    impl core::ops::Deref for RtcDataChannel {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for RtcDataChannel {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for RtcDataChannel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a RtcDataChannel {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for RtcDataChannel {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            RtcDataChannel {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for RtcDataChannel {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a RtcDataChannel {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for RtcDataChannel {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<RtcDataChannel>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(RtcDataChannel {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for RtcDataChannel {
        #[inline]
        fn from(obj: JsValue) -> RtcDataChannel {
            RtcDataChannel { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for RtcDataChannel {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<RtcDataChannel> for RtcDataChannel {
        #[inline]
        fn as_ref(&self) -> &RtcDataChannel {
            self
        }
    }
    impl From<RtcDataChannel> for JsValue {
        #[inline]
        fn from(obj: RtcDataChannel) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for RtcDataChannel {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_RTCDataChannel(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_RTCDataChannel(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_RTCDataChannel(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            RtcDataChannel { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const RtcDataChannel) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<RtcDataChannel> for EventTarget {
    #[inline]
    fn from(obj: RtcDataChannel) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for RtcDataChannel {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<RtcDataChannel> for ::js_sys::Object {
    #[inline]
    fn from(obj: RtcDataChannel) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for RtcDataChannel {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/close)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn close(&self) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_RTCDataChannel(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_str_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn send_with_str(&self, data: &str) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_str_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_str_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_str_RTCDataChannel(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Blob", feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_blob_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <&Blob as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "Blob", feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `Blob`, `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn send_with_blob(&self, data: &Blob) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Blob", feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_blob_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&Blob as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_blob_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&Blob as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_blob_RTCDataChannel(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer(
        &self,
        data: &::js_sys::ArrayBuffer,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_RTCDataChannel(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_array_buffer_view_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn send_with_array_buffer_view(
        &self,
        data: &::js_sys::Object,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_array_buffer_view_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_array_buffer_view_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_array_buffer_view_RTCDataChannel(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_send_with_u8_array_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `send()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/send)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn send_with_u8_array(&self, data: &mut [u8]) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_send_with_u8_array_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_send_with_u8_array_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let data = <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(data);
                __widl_f_send_with_u8_array_RTCDataChannel(self_, data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/label)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_RTCDataChannel(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reliable_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `reliable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/reliable)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn reliable(&self) -> bool {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reliable_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reliable_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reliable_RTCDataChannel(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_packet_life_time_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<u16> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `maxPacketLifeTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxPacketLifeTime)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn max_packet_life_time(&self) -> Option<u16> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_packet_life_time_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_packet_life_time_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_packet_life_time_RTCDataChannel(self_)
            };
            <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_retransmits_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<u16> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `maxRetransmits` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/maxRetransmits)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn max_retransmits(&self) -> Option<u16> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_retransmits_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_retransmits_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_retransmits_RTCDataChannel(self_)
            };
            <Option<u16> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <RtcDataChannelState as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelState",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/readyState)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelState`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> RtcDataChannelState {
        #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcDataChannelState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcDataChannelState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_RTCDataChannel(self_)
            };
            <RtcDataChannelState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffered_amount_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `bufferedAmount` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmount)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn buffered_amount(&self) -> u32 {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffered_amount_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffered_amount_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffered_amount_RTCDataChannel(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_buffered_amount_low_threshold_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `bufferedAmountLowThreshold` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn buffered_amount_low_threshold(&self) -> u32 {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_buffered_amount_low_threshold_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_buffered_amount_low_threshold_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_buffered_amount_low_threshold_RTCDataChannel(self_)
            };
            <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_buffered_amount_low_threshold_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `bufferedAmountLowThreshold` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/bufferedAmountLowThreshold)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn set_buffered_amount_low_threshold(&self, buffered_amount_low_threshold: u32) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_buffered_amount_low_threshold_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffered_amount_low_threshold: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_buffered_amount_low_threshold_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffered_amount_low_threshold: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(buffered_amount_low_threshold);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffered_amount_low_threshold =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        buffered_amount_low_threshold,
                    );
                __widl_f_set_buffered_amount_low_threshold_RTCDataChannel(
                    self_,
                    buffered_amount_low_threshold,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onopen_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn onopen(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onopen_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onopen_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onopen_RTCDataChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onopen_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onopen` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onopen)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onopen(&self, onopen: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onopen_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onopen_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onopen);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onopen =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onopen,
                    );
                __widl_f_set_onopen_RTCDataChannel(self_, onopen)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_RTCDataChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onerror)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_RTCDataChannel(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onclose_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn onclose(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onclose_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onclose_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onclose_RTCDataChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onclose_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onclose` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onclose)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onclose(&self, onclose: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onclose_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onclose: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onclose_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onclose =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onclose,
                    );
                __widl_f_set_onclose_RTCDataChannel(self_, onclose)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_RTCDataChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onmessage)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_RTCDataChannel(self_, onmessage)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onbufferedamountlow_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onbufferedamountlow` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn onbufferedamountlow(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onbufferedamountlow_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onbufferedamountlow_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onbufferedamountlow_RTCDataChannel(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onbufferedamountlow_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel",))]
    #[allow(bad_style)]
    #[doc = "The `onbufferedamountlow` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/onbufferedamountlow)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`*"]
    #[allow(clippy::all)]
    pub fn set_onbufferedamountlow(&self, onbufferedamountlow: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "RtcDataChannel",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onbufferedamountlow_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onbufferedamountlow : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onbufferedamountlow_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onbufferedamountlow : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onbufferedamountlow);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onbufferedamountlow =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onbufferedamountlow,
                    );
                __widl_f_set_onbufferedamountlow_RTCDataChannel(self_, onbufferedamountlow)
            };
            ()
        }
    }
}
#[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_binary_type_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <RtcDataChannelType as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelType",))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
    #[allow(clippy::all)]
    pub fn binary_type(&self) -> RtcDataChannelType {
        #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_binary_type_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RtcDataChannelType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_binary_type_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RtcDataChannelType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_binary_type_RTCDataChannel(self_)
            };
            <RtcDataChannelType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_binary_type_RTCDataChannel() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&RtcDataChannel as WasmDescribe>::describe();
    <RtcDataChannelType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl RtcDataChannel {
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelType",))]
    #[allow(bad_style)]
    #[doc = "The `binaryType` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannel/binaryType)\n\n*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelType`*"]
    #[allow(clippy::all)]
    pub fn set_binary_type(&self, binary_type: RtcDataChannelType) {
        #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_binary_type_RTCDataChannel(
                self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                binary_type: <RtcDataChannelType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_binary_type_RTCDataChannel(
            self_: <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            binary_type: <RtcDataChannelType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&RtcDataChannel as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let binary_type =
                    <RtcDataChannelType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        binary_type,
                    );
                __widl_f_set_binary_type_RTCDataChannel(self_, binary_type)
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
pub static __WASM_BINDGEN_GENERATED_7fd73751bcef1043: [u8; 2705usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}O\n\0\0\0\0\x1B\0\0\x02\x0ERTCDataChannel __widl_instanceof_RTCDataChannel\0\0\0\0\x1D__widl_f_close_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\0\x01\x01\x05self_\x05close\0\0\0%__widl_f_send_with_str_RTCDataChannel\x01\0\0\x01\x0ERTCDataChannel\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0&__widl_f_send_with_blob_RTCDataChannel\x01\0\0\x01\x0ERTCDataChannel\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0.__widl_f_send_with_array_buffer_RTCDataChannel\x01\0\0\x01\x0ERTCDataChannel\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\03__widl_f_send_with_array_buffer_view_RTCDataChannel\x01\0\0\x01\x0ERTCDataChannel\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0*__widl_f_send_with_u8_array_RTCDataChannel\x01\0\0\x01\x0ERTCDataChannel\x01\0\0\x01\x02\x05self_\x04data\x04send\0\0\0\x1D__widl_f_label_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0 __widl_f_reliable_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x08reliable\x01\x01\x05self_\x08reliable\0\0\0,__widl_f_max_packet_life_time_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x11maxPacketLifeTime\x01\x01\x05self_\x11maxPacketLifeTime\0\0\0'__widl_f_max_retransmits_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x0EmaxRetransmits\x01\x01\x05self_\x0EmaxRetransmits\0\0\0#__widl_f_ready_state_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0'__widl_f_buffered_amount_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x0EbufferedAmount\x01\x01\x05self_\x0EbufferedAmount\0\0\05__widl_f_buffered_amount_low_threshold_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x1AbufferedAmountLowThreshold\x01\x01\x05self_\x1AbufferedAmountLowThreshold\0\0\09__widl_f_set_buffered_amount_low_threshold_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\x1AbufferedAmountLowThreshold\x01\x02\x05self_\x1Dbuffered_amount_low_threshold\x1AbufferedAmountLowThreshold\0\0\0\x1E__widl_f_onopen_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x06onopen\x01\x01\x05self_\x06onopen\0\0\0\"__widl_f_set_onopen_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\x06onopen\x01\x02\x05self_\x06onopen\x06onopen\0\0\0\x1F__widl_f_onerror_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0#__widl_f_set_onerror_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1F__widl_f_onclose_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x07onclose\x01\x01\x05self_\x07onclose\0\0\0#__widl_f_set_onclose_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\x07onclose\x01\x02\x05self_\x07onclose\x07onclose\0\0\0!__widl_f_onmessage_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0%__widl_f_set_onmessage_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0+__widl_f_onbufferedamountlow_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\x13onbufferedamountlow\x01\x01\x05self_\x13onbufferedamountlow\0\0\0/__widl_f_set_onbufferedamountlow_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\x13onbufferedamountlow\x01\x02\x05self_\x13onbufferedamountlow\x13onbufferedamountlow\0\0\0#__widl_f_binary_type_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x01\nbinaryType\x01\x01\x05self_\nbinaryType\0\0\0'__widl_f_set_binary_type_RTCDataChannel\0\0\0\x01\x0ERTCDataChannel\x01\0\x02\nbinaryType\x01\x02\x05self_\x0Bbinary_type\nbinaryType\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
