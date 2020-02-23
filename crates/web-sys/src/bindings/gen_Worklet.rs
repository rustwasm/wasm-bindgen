use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Worklet` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet)\n\n*This API requires the following crate features to be activated: `Worklet`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Worklet {
    obj: ::js_sys::Object,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Worklet: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Worklet {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(7u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(108u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for Worklet {
        type Target = ::js_sys::Object;
        #[inline]
        fn deref(&self) -> &::js_sys::Object {
            &self.obj
        }
    }
    impl IntoWasmAbi for Worklet {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Worklet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Worklet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Worklet {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Worklet {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Worklet {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Worklet {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Worklet {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Worklet>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Worklet {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Worklet {
        #[inline]
        fn from(obj: JsValue) -> Worklet {
            Worklet { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Worklet {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Worklet> for Worklet {
        #[inline]
        fn as_ref(&self) -> &Worklet {
            self
        }
    }
    impl From<Worklet> for JsValue {
        #[inline]
        fn from(obj: Worklet) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Worklet {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Worklet(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Worklet(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Worklet(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Worklet { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Worklet) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Worklet> for ::js_sys::Object {
    #[inline]
    fn from(obj: Worklet) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Worklet {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Worklet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_module_Worklet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Worklet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Worklet {
    #[cfg(all(feature = "Worklet",))]
    #[allow(bad_style)]
    #[doc = "The `addModule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)\n\n*This API requires the following crate features to be activated: `Worklet`*"]
    #[allow(clippy::all)]
    pub fn add_module(
        &self,
        module_url: &str,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Worklet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_module_Worklet(
                self_: <&Worklet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                module_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_module_Worklet(
            self_: <&Worklet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            module_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(module_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worklet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let module_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(module_url);
                __widl_f_add_module_Worklet(self_, module_url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Worklet", feature = "WorkletOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_module_with_options_Worklet() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&Worklet as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <&WorkletOptions as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Worklet {
    #[cfg(all(feature = "Worklet", feature = "WorkletOptions",))]
    #[allow(bad_style)]
    #[doc = "The `addModule()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worklet/addModule)\n\n*This API requires the following crate features to be activated: `Worklet`, `WorkletOptions`*"]
    #[allow(clippy::all)]
    pub fn add_module_with_options(
        &self,
        module_url: &str,
        options: &WorkletOptions,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Worklet", feature = "WorkletOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_module_with_options_Worklet(
                self_: <&Worklet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                module_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&WorkletOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_module_with_options_Worklet(
            self_: <&Worklet as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            module_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&WorkletOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(module_url);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Worklet as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let module_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(module_url);
                let options =
                    <&WorkletOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_add_module_with_options_Worklet(self_, module_url, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_0c868c52b0a9b65c: [u8; 314usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xF8\0\0\0\0\0\x03\0\0\x02\x07Worklet\x19__widl_instanceof_Worklet\0\0\0\0\x1B__widl_f_add_module_Worklet\x01\0\0\x01\x07Worklet\x01\0\0\x01\x02\x05self_\nmodule_url\taddModule\0\0\0(__widl_f_add_module_with_options_Worklet\x01\0\0\x01\x07Worklet\x01\0\0\x01\x03\x05self_\nmodule_url\x07options\taddModule\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
