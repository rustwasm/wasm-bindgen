use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `ChromeWorker` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChromeWorker)\n\n*This API requires the following crate features to be activated: `ChromeWorker`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct ChromeWorker {
    obj: Worker,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_ChromeWorker: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for ChromeWorker {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(67u32);
            inform(104u32);
            inform(114u32);
            inform(111u32);
            inform(109u32);
            inform(101u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for ChromeWorker {
        type Target = Worker;
        #[inline]
        fn deref(&self) -> &Worker {
            &self.obj
        }
    }
    impl IntoWasmAbi for ChromeWorker {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for ChromeWorker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a ChromeWorker {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for ChromeWorker {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            ChromeWorker {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for ChromeWorker {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a ChromeWorker {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for ChromeWorker {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<ChromeWorker>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(ChromeWorker {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for ChromeWorker {
        #[inline]
        fn from(obj: JsValue) -> ChromeWorker {
            ChromeWorker { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for ChromeWorker {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<ChromeWorker> for ChromeWorker {
        #[inline]
        fn as_ref(&self) -> &ChromeWorker {
            self
        }
    }
    impl From<ChromeWorker> for JsValue {
        #[inline]
        fn from(obj: ChromeWorker) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for ChromeWorker {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_ChromeWorker(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_ChromeWorker(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_ChromeWorker(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            ChromeWorker { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const ChromeWorker) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<ChromeWorker> for Worker {
    #[inline]
    fn from(obj: ChromeWorker) -> Worker {
        use wasm_bindgen::JsCast;
        Worker::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Worker> for ChromeWorker {
    #[inline]
    fn as_ref(&self) -> &Worker {
        use wasm_bindgen::JsCast;
        Worker::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ChromeWorker> for EventTarget {
    #[inline]
    fn from(obj: ChromeWorker) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for ChromeWorker {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<ChromeWorker> for ::js_sys::Object {
    #[inline]
    fn from(obj: ChromeWorker) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for ChromeWorker {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "ChromeWorker",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_ChromeWorker() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <ChromeWorker as WasmDescribe>::describe();
}
impl ChromeWorker {
    #[cfg(all(feature = "ChromeWorker",))]
    #[allow(bad_style)]
    #[doc = "The `new ChromeWorker(..)` constructor, creating a new instance of `ChromeWorker`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChromeWorker/ChromeWorker)\n\n*This API requires the following crate features to be activated: `ChromeWorker`*"]
    #[allow(clippy::all)]
    pub fn new(script_url: &str) -> Result<ChromeWorker, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "ChromeWorker",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_ChromeWorker(
                script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChromeWorker as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_ChromeWorker(
            script_url: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChromeWorker as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(script_url);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let script_url = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(script_url);
                __widl_f_new_ChromeWorker(script_url)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChromeWorker as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_1118ff08ae43c004: [u8; 217usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x97\0\0\0\0\0\x02\0\0\x02\x0CChromeWorker\x1E__widl_instanceof_ChromeWorker\0\0\0\0\x19__widl_f_new_ChromeWorker\x01\0\0\x01\x0CChromeWorker\0\x01\x01\nscript_url\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
