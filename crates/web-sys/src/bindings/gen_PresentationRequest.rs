use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PresentationRequest` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PresentationRequest {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PresentationRequest: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PresentationRequest {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
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
            inform(82u32);
            inform(101u32);
            inform(113u32);
            inform(117u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for PresentationRequest {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for PresentationRequest {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PresentationRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PresentationRequest {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PresentationRequest {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PresentationRequest {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PresentationRequest {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PresentationRequest {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PresentationRequest {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PresentationRequest>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PresentationRequest {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PresentationRequest {
        #[inline]
        fn from(obj: JsValue) -> PresentationRequest {
            PresentationRequest { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PresentationRequest {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PresentationRequest> for PresentationRequest {
        #[inline]
        fn as_ref(&self) -> &PresentationRequest {
            self
        }
    }
    impl From<PresentationRequest> for JsValue {
        #[inline]
        fn from(obj: PresentationRequest) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PresentationRequest {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PresentationRequest(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PresentationRequest(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PresentationRequest(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PresentationRequest { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PresentationRequest) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PresentationRequest> for EventTarget {
    #[inline]
    fn from(obj: PresentationRequest) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for PresentationRequest {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PresentationRequest> for ::js_sys::Object {
    #[inline]
    fn from(obj: PresentationRequest) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PresentationRequest {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_url_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <PresentationRequest as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn new_with_url(url: &str) -> Result<PresentationRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_url_PresentationRequest(
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PresentationRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_url_PresentationRequest(
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PresentationRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_new_with_url_PresentationRequest(url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PresentationRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_urls_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <PresentationRequest as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn new_with_urls(
        urls: &::wasm_bindgen::JsValue,
    ) -> Result<PresentationRequest, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_urls_PresentationRequest(
                urls: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PresentationRequest as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_urls_PresentationRequest(
            urls: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PresentationRequest as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(urls);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let urls =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        urls,
                    );
                __widl_f_new_with_urls_PresentationRequest(urls)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PresentationRequest as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_availability_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationRequest as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `getAvailability()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn get_availability(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_availability_PresentationRequest(
                self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_availability_PresentationRequest(
            self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_availability_PresentationRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reconnect_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationRequest as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `reconnect()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn reconnect(
        &self,
        presentation_id: &str,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reconnect_PresentationRequest(
                self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                presentation_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reconnect_PresentationRequest(
            self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            presentation_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(presentation_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let presentation_id =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(presentation_id);
                __widl_f_reconnect_PresentationRequest(self_, presentation_id)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationRequest as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_PresentationRequest(
                self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_PresentationRequest(
            self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_PresentationRequest(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onconnectionavailable_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PresentationRequest as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onconnectionavailable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn onconnectionavailable(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onconnectionavailable_PresentationRequest(
                self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onconnectionavailable_PresentationRequest(
            self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                    <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onconnectionavailable_PresentationRequest(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PresentationRequest",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onconnectionavailable_PresentationRequest() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PresentationRequest as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PresentationRequest {
    #[cfg(all(feature = "PresentationRequest",))]
    #[allow(bad_style)]
    #[doc = "The `onconnectionavailable` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)\n\n*This API requires the following crate features to be activated: `PresentationRequest`*"]
    #[allow(clippy::all)]
    pub fn set_onconnectionavailable(&self, onconnectionavailable: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "PresentationRequest",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onconnectionavailable_PresentationRequest(
                self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onconnectionavailable : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onconnectionavailable_PresentationRequest(
            self_: <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onconnectionavailable : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onconnectionavailable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&PresentationRequest as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onconnectionavailable =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onconnectionavailable,
                    );
                __widl_f_set_onconnectionavailable_PresentationRequest(self_, onconnectionavailable)
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
pub static __WASM_BINDGEN_GENERATED_02a0725a75e6478e: [u8; 903usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}E\x03\0\0\0\0\x08\0\0\x02\x13PresentationRequest%__widl_instanceof_PresentationRequest\0\0\0\0)__widl_f_new_with_url_PresentationRequest\x01\0\0\x01\x13PresentationRequest\0\x01\x01\x03url\x03new\0\0\0*__widl_f_new_with_urls_PresentationRequest\x01\0\0\x01\x13PresentationRequest\0\x01\x01\x04urls\x03new\0\0\0-__widl_f_get_availability_PresentationRequest\x01\0\0\x01\x13PresentationRequest\x01\0\0\x01\x01\x05self_\x0FgetAvailability\0\0\0&__widl_f_reconnect_PresentationRequest\x01\0\0\x01\x13PresentationRequest\x01\0\0\x01\x02\x05self_\x0Fpresentation_id\treconnect\0\0\0\"__widl_f_start_PresentationRequest\x01\0\0\x01\x13PresentationRequest\x01\0\0\x01\x01\x05self_\x05start\0\0\02__widl_f_onconnectionavailable_PresentationRequest\0\0\0\x01\x13PresentationRequest\x01\0\x01\x15onconnectionavailable\x01\x01\x05self_\x15onconnectionavailable\0\0\06__widl_f_set_onconnectionavailable_PresentationRequest\0\0\0\x01\x13PresentationRequest\x01\0\x02\x15onconnectionavailable\x01\x02\x05self_\x15onconnectionavailable\x15onconnectionavailable\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
