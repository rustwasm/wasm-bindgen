use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Clients` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients)\n\n*This API requires the following crate features to be activated: `Clients`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Clients {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Clients: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Clients {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(67u32);
            inform(108u32);
            inform(105u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
            inform(115u32);
        }
    }
    impl core::ops::Deref for Clients {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Clients {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Clients {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Clients {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Clients {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Clients {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Clients {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Clients {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Clients {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Clients>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Clients {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Clients {
        #[inline]
        fn from(obj: JsValue) -> Clients {
            Clients { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Clients {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Clients> for Clients {
        #[inline]
        fn as_ref(&self) -> &Clients {
            self
        }
    }
    impl From<Clients> for JsValue {
        #[inline]
        fn from(obj: Clients) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Clients {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Clients(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Clients(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Clients(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Clients { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Clients) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Clients> for ::js_sys::Object {
    #[inline]
    fn from(obj: Clients) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Clients {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Clients",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_claim_Clients() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Clients as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Clients {
    #[cfg(all(feature = "Clients",))]
    #[allow(bad_style)]
    #[doc = "The `claim()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/claim)\n\n*This API requires the following crate features to be activated: `Clients`*"]
    #[allow(clippy::all)]
    pub fn claim(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "Clients",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_claim_Clients(
                self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_claim_Clients(
            self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Clients as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_claim_Clients(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Clients",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_Clients() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Clients as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Clients {
    #[cfg(all(feature = "Clients",))]
    #[allow(bad_style)]
    #[doc = "The `get()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/get)\n\n*This API requires the following crate features to be activated: `Clients`*"]
    #[allow(clippy::all)]
    pub fn get(&self, id: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Clients",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_Clients(
                self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_Clients(
            self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Clients as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_get_Clients(self_, id)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Clients",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_Clients() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Clients as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Clients {
    #[cfg(all(feature = "Clients",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)\n\n*This API requires the following crate features to be activated: `Clients`*"]
    #[allow(clippy::all)]
    pub fn match_all(&self) -> ::js_sys::Promise {
        #[cfg(all(feature = "Clients",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_Clients(
                self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_Clients(
            self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Clients as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_match_all_Clients(self_)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "ClientQueryOptions", feature = "Clients",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_match_all_with_options_Clients() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Clients as WasmDescribe>::describe();
    <&ClientQueryOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Clients {
    #[cfg(all(feature = "ClientQueryOptions", feature = "Clients",))]
    #[allow(bad_style)]
    #[doc = "The `matchAll()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)\n\n*This API requires the following crate features to be activated: `ClientQueryOptions`, `Clients`*"]
    #[allow(clippy::all)]
    pub fn match_all_with_options(&self, options: &ClientQueryOptions) -> ::js_sys::Promise {
        #[cfg(all(feature = "ClientQueryOptions", feature = "Clients",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_match_all_with_options_Clients(
                self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&ClientQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_match_all_with_options_Clients(
            self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&ClientQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Clients as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let options =
                    <&ClientQueryOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_match_all_with_options_Clients(self_, options)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Clients",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_open_window_Clients() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Clients as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Clients {
    #[cfg(all(feature = "Clients",))]
    #[allow(bad_style)]
    #[doc = "The `openWindow()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/openWindow)\n\n*This API requires the following crate features to be activated: `Clients`*"]
    #[allow(clippy::all)]
    pub fn open_window(&self, url: &str) -> ::js_sys::Promise {
        #[cfg(all(feature = "Clients",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_open_window_Clients(
                self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_open_window_Clients(
            self_: <&Clients as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Clients as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(url);
                __widl_f_open_window_Clients(self_, url)
            };
            <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e90a898f3aefe288: [u8; 467usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x91\x01\0\0\0\0\x06\0\0\x02\x07Clients\x19__widl_instanceof_Clients\0\0\0\0\x16__widl_f_claim_Clients\0\0\0\x01\x07Clients\x01\0\0\x01\x01\x05self_\x05claim\0\0\0\x14__widl_f_get_Clients\0\0\0\x01\x07Clients\x01\0\0\x01\x02\x05self_\x02id\x03get\0\0\0\x1A__widl_f_match_all_Clients\0\0\0\x01\x07Clients\x01\0\0\x01\x01\x05self_\x08matchAll\0\0\0'__widl_f_match_all_with_options_Clients\0\0\0\x01\x07Clients\x01\0\0\x01\x02\x05self_\x07options\x08matchAll\0\0\0\x1C__widl_f_open_window_Clients\0\0\0\x01\x07Clients\x01\0\0\x01\x02\x05self_\x03url\nopenWindow\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
