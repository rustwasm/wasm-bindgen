use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `WindowClient` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct WindowClient {
    obj: Client,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_WindowClient: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for WindowClient {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(87u32);
            inform(105u32);
            inform(110u32);
            inform(100u32);
            inform(111u32);
            inform(119u32);
            inform(67u32);
            inform(108u32);
            inform(105u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for WindowClient {
        type Target = Client;
        #[inline]
        fn deref(&self) -> &Client {
            &self.obj
        }
    }
    impl IntoWasmAbi for WindowClient {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for WindowClient {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a WindowClient {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for WindowClient {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            WindowClient {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for WindowClient {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a WindowClient {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for WindowClient {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<WindowClient>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(WindowClient {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for WindowClient {
        #[inline]
        fn from(obj: JsValue) -> WindowClient {
            WindowClient { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for WindowClient {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<WindowClient> for WindowClient {
        #[inline]
        fn as_ref(&self) -> &WindowClient {
            self
        }
    }
    impl From<WindowClient> for JsValue {
        #[inline]
        fn from(obj: WindowClient) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for WindowClient {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_WindowClient(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_WindowClient(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_WindowClient(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            WindowClient { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const WindowClient) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<WindowClient> for Client {
    #[inline]
    fn from(obj: WindowClient) -> Client {
        use wasm_bindgen::JsCast;
        Client::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Client> for WindowClient {
    #[inline]
    fn as_ref(&self) -> &Client {
        use wasm_bindgen::JsCast;
        Client::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<WindowClient> for ::js_sys::Object {
    #[inline]
    fn from(obj: WindowClient) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for WindowClient {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "WindowClient",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focus_WindowClient() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WindowClient as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WindowClient {
    #[cfg(all(feature = "WindowClient",))]
    #[allow(bad_style)]
    #[doc = "The `focus()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focus)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    #[allow(clippy::all)]
    pub fn focus(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WindowClient",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focus_WindowClient(
                self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focus_WindowClient(
            self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focus_WindowClient(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "WindowClient",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_navigate_WindowClient() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&WindowClient as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl WindowClient {
    #[cfg(all(feature = "WindowClient",))]
    #[allow(bad_style)]
    #[doc = "The `navigate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/navigate)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    #[allow(clippy::all)]
    pub fn navigate(&self, url: &str) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "WindowClient",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_navigate_WindowClient(
                self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_navigate_WindowClient(
            self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_navigate_WindowClient(self_, url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "VisibilityState", feature = "WindowClient",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_visibility_state_WindowClient() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WindowClient as WasmDescribe>::describe();
    <VisibilityState as WasmDescribe>::describe();
}
impl WindowClient {
    #[cfg(all(feature = "VisibilityState", feature = "WindowClient",))]
    #[allow(bad_style)]
    #[doc = "The `visibilityState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/visibilityState)\n\n*This API requires the following crate features to be activated: `VisibilityState`, `WindowClient`*"]
    #[allow(clippy::all)]
    pub fn visibility_state(&self) -> VisibilityState {
        #[cfg(all(feature = "VisibilityState", feature = "WindowClient",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_visibility_state_WindowClient(
                self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <VisibilityState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_visibility_state_WindowClient(
            self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <VisibilityState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_visibility_state_WindowClient(self_)
            };
            <VisibilityState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "WindowClient",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_focused_WindowClient() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&WindowClient as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl WindowClient {
    #[cfg(all(feature = "WindowClient",))]
    #[allow(bad_style)]
    #[doc = "The `focused` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WindowClient/focused)\n\n*This API requires the following crate features to be activated: `WindowClient`*"]
    #[allow(clippy::all)]
    pub fn focused(&self) -> bool {
        #[cfg(all(feature = "WindowClient",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_focused_WindowClient(
                self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_focused_WindowClient(
            self_: <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&WindowClient as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_focused_WindowClient(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1b74b293e2dbc62c: [u8; 472usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x96\x01\0\0\0\0\x05\0\0\x02\x0CWindowClient\x1E__widl_instanceof_WindowClient\0\0\0\0\x1B__widl_f_focus_WindowClient\x01\0\0\x01\x0CWindowClient\x01\0\0\x01\x01\x05self_\x05focus\0\0\0\x1E__widl_f_navigate_WindowClient\x01\0\0\x01\x0CWindowClient\x01\0\0\x01\x02\x05self_\x03url\x08navigate\0\0\0&__widl_f_visibility_state_WindowClient\0\0\0\x01\x0CWindowClient\x01\0\x01\x0FvisibilityState\x01\x01\x05self_\x0FvisibilityState\0\0\0\x1D__widl_f_focused_WindowClient\0\0\0\x01\x0CWindowClient\x01\0\x01\x07focused\x01\x01\x05self_\x07focused\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
