use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaKeySession` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaKeySession {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaKeySession: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaKeySession {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(15u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(83u32);
            inform(101u32);
            inform(115u32);
            inform(115u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for MediaKeySession {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaKeySession {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaKeySession {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaKeySession {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaKeySession {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaKeySession {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaKeySession {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaKeySession {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaKeySession {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaKeySession>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaKeySession {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaKeySession {
        #[inline]
        fn from(obj: JsValue) -> MediaKeySession {
            MediaKeySession { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaKeySession {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaKeySession> for MediaKeySession {
        #[inline]
        fn as_ref(&self) -> &MediaKeySession {
            self
        }
    }
    impl From<MediaKeySession> for JsValue {
        #[inline]
        fn from(obj: MediaKeySession) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaKeySession {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaKeySession(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaKeySession(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaKeySession(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaKeySession { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaKeySession) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaKeySession> for EventTarget {
    #[inline]
    fn from(obj: MediaKeySession) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaKeySession {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaKeySession> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaKeySession) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaKeySession {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/close)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn close(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_MediaKeySession(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_request_with_buffer_source_MediaKeySession()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `generateRequest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn generate_request_with_buffer_source(
        &self,
        init_data_type: &str,
        init_data: &::js_sys::Object,
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_request_with_buffer_source_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init_data_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init_data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_request_with_buffer_source_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init_data_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init_data: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(init_data_type);
            drop(init_data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let init_data_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init_data_type);
                let init_data =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init_data);
                __widl_f_generate_request_with_buffer_source_MediaKeySession(
                    self_,
                    init_data_type,
                    init_data,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_generate_request_with_u8_array_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `generateRequest()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn generate_request_with_u8_array(
        &self,
        init_data_type: &str,
        init_data: &mut [u8],
    ) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_generate_request_with_u8_array_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init_data_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                init_data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_generate_request_with_u8_array_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init_data_type: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            init_data: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(init_data_type);
            drop(init_data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let init_data_type =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init_data_type);
                let init_data =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(init_data);
                __widl_f_generate_request_with_u8_array_MediaKeySession(
                    self_,
                    init_data_type,
                    init_data,
                )
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_load_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `load()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/load)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn load(&self, session_id: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_load_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                session_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_load_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            session_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(session_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let session_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(session_id);
                __widl_f_load_MediaKeySession(self_, session_id)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `remove()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/remove)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn remove(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_remove_MediaKeySession(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_with_buffer_source_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <&::js_sys::Object as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `update()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn update_with_buffer_source(&self, response: &::js_sys::Object) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_with_buffer_source_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                response: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_with_buffer_source_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            response: <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(response);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let response =
                    <&::js_sys::Object as wasm_bindgen::convert::IntoWasmAbi>::into_abi(response);
                __widl_f_update_with_buffer_source_MediaKeySession(self_, response)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_with_u8_array_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <&mut [u8] as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `update()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn update_with_u8_array(&self, response: &mut [u8]) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_with_u8_array_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                response: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_with_u8_array_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            response: <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(response);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let response =
                    <&mut [u8] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(response);
                __widl_f_update_with_u8_array_MediaKeySession(self_, response)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeyError", feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_error_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <Option<MediaKeyError> as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeyError", feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `error` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/error)\n\n*This API requires the following crate features to be activated: `MediaKeyError`, `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn error(&self) -> Option<MediaKeyError> {
        #[cfg(all(feature = "MediaKeyError", feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_error_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaKeyError> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_error_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaKeyError> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_error_MediaKeySession(self_)
            };
            <Option<MediaKeyError> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_session_id_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `sessionId` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/sessionId)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn session_id(&self) -> String {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_session_id_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_session_id_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_session_id_MediaKeySession(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_expiration_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `expiration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/expiration)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn expiration(&self) -> f64 {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_expiration_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_expiration_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_expiration_MediaKeySession(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_closed_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `closed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/closed)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn closed(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_closed_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_closed_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_closed_MediaKeySession(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession", feature = "MediaKeyStatusMap",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_key_statuses_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <MediaKeyStatusMap as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession", feature = "MediaKeyStatusMap",))]
    #[allow(bad_style)]
    #[doc = "The `keyStatuses` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/keyStatuses)\n\n*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeyStatusMap`*"]
    #[allow(clippy::all)]
    pub fn key_statuses(&self) -> MediaKeyStatusMap {
        #[cfg(all(feature = "MediaKeySession", feature = "MediaKeyStatusMap",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_key_statuses_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaKeyStatusMap as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_key_statuses_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaKeyStatusMap as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_key_statuses_MediaKeySession(self_)
            };
            <MediaKeyStatusMap as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onkeystatuseschange_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `onkeystatuseschange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn onkeystatuseschange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onkeystatuseschange_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onkeystatuseschange_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onkeystatuseschange_MediaKeySession(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onkeystatuseschange_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `onkeystatuseschange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn set_onkeystatuseschange(&self, onkeystatuseschange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onkeystatuseschange_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onkeystatuseschange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onkeystatuseschange_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onkeystatuseschange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onkeystatuseschange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onkeystatuseschange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onkeystatuseschange,
                    );
                __widl_f_set_onkeystatuseschange_MediaKeySession(self_, onkeystatuseschange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onmessage_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn onmessage(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onmessage_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onmessage_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onmessage_MediaKeySession(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaKeySession",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onmessage_MediaKeySession() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaKeySession as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaKeySession {
    #[cfg(all(feature = "MediaKeySession",))]
    #[allow(bad_style)]
    #[doc = "The `onmessage` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)\n\n*This API requires the following crate features to be activated: `MediaKeySession`*"]
    #[allow(clippy::all)]
    pub fn set_onmessage(&self, onmessage: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaKeySession",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onmessage_MediaKeySession(
                self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onmessage: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onmessage_MediaKeySession(
            self_: <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&MediaKeySession as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onmessage =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onmessage,
                    );
                __widl_f_set_onmessage_MediaKeySession(self_, onmessage)
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
pub static __WASM_BINDGEN_GENERATED_43b91d0caa59d96d: [u8; 1736usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x86\x06\0\0\0\0\x11\0\0\x02\x0FMediaKeySession!__widl_instanceof_MediaKeySession\0\0\0\0\x1E__widl_f_close_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x01\x05self_\x05close\0\0\0<__widl_f_generate_request_with_buffer_source_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x03\x05self_\x0Einit_data_type\tinit_data\x0FgenerateRequest\0\0\07__widl_f_generate_request_with_u8_array_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x03\x05self_\x0Einit_data_type\tinit_data\x0FgenerateRequest\0\0\0\x1D__widl_f_load_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x02\x05self_\nsession_id\x04load\0\0\0\x1F__widl_f_remove_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x01\x05self_\x06remove\0\0\02__widl_f_update_with_buffer_source_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x02\x05self_\x08response\x06update\0\0\0-__widl_f_update_with_u8_array_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\0\x01\x02\x05self_\x08response\x06update\0\0\0\x1E__widl_f_error_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\x05error\x01\x01\x05self_\x05error\0\0\0#__widl_f_session_id_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\tsessionId\x01\x01\x05self_\tsessionId\0\0\0#__widl_f_expiration_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\nexpiration\x01\x01\x05self_\nexpiration\0\0\0\x1F__widl_f_closed_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\x06closed\x01\x01\x05self_\x06closed\0\0\0%__widl_f_key_statuses_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\x0BkeyStatuses\x01\x01\x05self_\x0BkeyStatuses\0\0\0,__widl_f_onkeystatuseschange_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\x13onkeystatuseschange\x01\x01\x05self_\x13onkeystatuseschange\0\0\00__widl_f_set_onkeystatuseschange_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x02\x13onkeystatuseschange\x01\x02\x05self_\x13onkeystatuseschange\x13onkeystatuseschange\0\0\0\"__widl_f_onmessage_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x01\tonmessage\x01\x01\x05self_\tonmessage\0\0\0&__widl_f_set_onmessage_MediaKeySession\0\0\0\x01\x0FMediaKeySession\x01\0\x02\tonmessage\x01\x02\x05self_\tonmessage\tonmessage\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
