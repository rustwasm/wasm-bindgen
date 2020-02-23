use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `U2F` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F)\n\n*This API requires the following crate features to be activated: `U2f`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct U2f {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_U2f: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for U2f {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(3u32);
            inform(85u32);
            inform(50u32);
            inform(70u32);
        }
    }
    impl core::ops::Deref for U2f {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for U2f {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for U2f {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a U2f {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for U2f {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            U2f {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for U2f {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a U2f {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for U2f {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<U2f>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(U2f {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for U2f {
        #[inline]
        fn from(obj: JsValue) -> U2f {
            U2f { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for U2f {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<U2f> for U2f {
        #[inline]
        fn as_ref(&self) -> &U2f {
            self
        }
    }
    impl From<U2f> for JsValue {
        #[inline]
        fn from(obj: U2f) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for U2f {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_U2F(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_U2F(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_U2F(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            U2f { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const U2f) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<U2f> for ::js_sys::Object {
    #[inline]
    fn from(obj: U2f) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for U2f {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "U2f",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_U2F() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&U2f as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl U2f {
    #[cfg(all(feature = "U2f",))]
    #[allow(bad_style)]
    #[doc = "The `register()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/register)\n\n*This API requires the following crate features to be activated: `U2f`*"]
    #[allow(clippy::all)]
    pub fn register(
        &self,
        app_id: &str,
        register_requests: &::wasm_bindgen::JsValue,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "U2f",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_U2F(
                self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                register_requests : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                registered_keys : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_U2F(
            self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            register_requests : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            registered_keys: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(app_id);
            drop(register_requests);
            drop(registered_keys);
            drop(callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&U2f as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let app_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(app_id);
                let register_requests =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        register_requests,
                    );
                let registered_keys =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        registered_keys,
                    );
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                __widl_f_register_U2F(self_, app_id, register_requests, registered_keys, callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "U2f",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_register_with_opt_timeout_seconds_U2F() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&U2f as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <Option<i32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl U2f {
    #[cfg(all(feature = "U2f",))]
    #[allow(bad_style)]
    #[doc = "The `register()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/register)\n\n*This API requires the following crate features to be activated: `U2f`*"]
    #[allow(clippy::all)]
    pub fn register_with_opt_timeout_seconds(
        &self,
        app_id: &str,
        register_requests: &::wasm_bindgen::JsValue,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
        opt_timeout_seconds: Option<i32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "U2f",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_register_with_opt_timeout_seconds_U2F(
                self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                register_requests : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                registered_keys : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                opt_timeout_seconds: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_register_with_opt_timeout_seconds_U2F(
            self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            register_requests : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            registered_keys: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            opt_timeout_seconds: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(app_id);
            drop(register_requests);
            drop(registered_keys);
            drop(callback);
            drop(opt_timeout_seconds);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&U2f as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let app_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(app_id);
                let register_requests =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        register_requests,
                    );
                let registered_keys =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        registered_keys,
                    );
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                let opt_timeout_seconds =
                    <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        opt_timeout_seconds,
                    );
                __widl_f_register_with_opt_timeout_seconds_U2F(
                    self_,
                    app_id,
                    register_requests,
                    registered_keys,
                    callback,
                    opt_timeout_seconds,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "U2f",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sign_U2F() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(5u32);
    <&U2f as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl U2f {
    #[cfg(all(feature = "U2f",))]
    #[allow(bad_style)]
    #[doc = "The `sign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/sign)\n\n*This API requires the following crate features to be activated: `U2f`*"]
    #[allow(clippy::all)]
    pub fn sign(
        &self,
        app_id: &str,
        challenge: &str,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "U2f",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sign_U2F(
                self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                challenge: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                registered_keys : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sign_U2F(
            self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            challenge: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            registered_keys: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(app_id);
            drop(challenge);
            drop(registered_keys);
            drop(callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&U2f as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let app_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(app_id);
                let challenge = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(challenge);
                let registered_keys =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        registered_keys,
                    );
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                __widl_f_sign_U2F(self_, app_id, challenge, registered_keys, callback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "U2f",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sign_with_opt_timeout_seconds_U2F() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(6u32);
    <&U2f as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <Option<i32> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl U2f {
    #[cfg(all(feature = "U2f",))]
    #[allow(bad_style)]
    #[doc = "The `sign()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/U2F/sign)\n\n*This API requires the following crate features to be activated: `U2f`*"]
    #[allow(clippy::all)]
    pub fn sign_with_opt_timeout_seconds(
        &self,
        app_id: &str,
        challenge: &str,
        registered_keys: &::wasm_bindgen::JsValue,
        callback: &::js_sys::Function,
        opt_timeout_seconds: Option<i32>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "U2f",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sign_with_opt_timeout_seconds_U2F(
                self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                challenge: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                registered_keys : < & :: wasm_bindgen :: JsValue as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
                callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                opt_timeout_seconds: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sign_with_opt_timeout_seconds_U2F(
            self_: <&U2f as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            app_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            challenge: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            registered_keys: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            opt_timeout_seconds: <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(app_id);
            drop(challenge);
            drop(registered_keys);
            drop(callback);
            drop(opt_timeout_seconds);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&U2f as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let app_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(app_id);
                let challenge = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(challenge);
                let registered_keys =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        registered_keys,
                    );
                let callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(callback);
                let opt_timeout_seconds =
                    <Option<i32> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        opt_timeout_seconds,
                    );
                __widl_f_sign_with_opt_timeout_seconds_U2F(
                    self_,
                    app_id,
                    challenge,
                    registered_keys,
                    callback,
                    opt_timeout_seconds,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
impl U2f {
    pub const OK: u16 = 0i64 as u16;
}
impl U2f {
    pub const OTHER_ERROR: u16 = 1u64 as u16;
}
impl U2f {
    pub const BAD_REQUEST: u16 = 2u64 as u16;
}
impl U2f {
    pub const CONFIGURATION_UNSUPPORTED: u16 = 3u64 as u16;
}
impl U2f {
    pub const DEVICE_INELIGIBLE: u16 = 4u64 as u16;
}
impl U2f {
    pub const TIMEOUT: u16 = 5u64 as u16;
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_914488e3cc582fb8: [u8; 605usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x1B\x02\0\0\0\0\x05\0\0\x02\x03U2F\x15__widl_instanceof_U2F\0\0\0\0\x15__widl_f_register_U2F\x01\0\0\x01\x03U2F\x01\0\0\x01\x05\x05self_\x06app_id\x11register_requests\x0Fregistered_keys\x08callback\x08register\0\0\0.__widl_f_register_with_opt_timeout_seconds_U2F\x01\0\0\x01\x03U2F\x01\0\0\x01\x06\x05self_\x06app_id\x11register_requests\x0Fregistered_keys\x08callback\x13opt_timeout_seconds\x08register\0\0\0\x11__widl_f_sign_U2F\x01\0\0\x01\x03U2F\x01\0\0\x01\x05\x05self_\x06app_id\tchallenge\x0Fregistered_keys\x08callback\x04sign\0\0\0*__widl_f_sign_with_opt_timeout_seconds_U2F\x01\0\0\x01\x03U2F\x01\0\0\x01\x06\x05self_\x06app_id\tchallenge\x0Fregistered_keys\x08callback\x13opt_timeout_seconds\x04sign\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
