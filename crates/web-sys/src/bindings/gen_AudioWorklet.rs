use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioWorklet` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet)\n\n*This API requires the following crate features to be activated: `AudioWorklet`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioWorklet {
    obj: Worklet,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioWorklet: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioWorklet {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(87u32);
            inform(111u32);
            inform(114u32);
            inform(107u32);
            inform(108u32);
            inform(101u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for AudioWorklet {
        type Target = Worklet;
        #[inline]
        fn deref(&self) -> &Worklet {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioWorklet {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioWorklet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioWorklet {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioWorklet {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioWorklet {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioWorklet {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioWorklet {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioWorklet {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioWorklet>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioWorklet {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioWorklet {
        #[inline]
        fn from(obj: JsValue) -> AudioWorklet {
            AudioWorklet { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioWorklet {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioWorklet> for AudioWorklet {
        #[inline]
        fn as_ref(&self) -> &AudioWorklet {
            self
        }
    }
    impl From<AudioWorklet> for JsValue {
        #[inline]
        fn from(obj: AudioWorklet) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioWorklet {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioWorklet(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioWorklet(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioWorklet(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioWorklet { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioWorklet) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioWorklet> for Worklet {
    #[inline]
    fn from(obj: AudioWorklet) -> Worklet {
        use wasm_bindgen::JsCast;
        Worklet::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<Worklet> for AudioWorklet {
    #[inline]
    fn as_ref(&self) -> &Worklet {
        use wasm_bindgen::JsCast;
        Worklet::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioWorklet> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioWorklet) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioWorklet {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_ca46f8b98c72f35e: [u8; 153usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}W\0\0\0\0\0\x01\0\0\x02\x0CAudioWorklet\x1E__widl_instanceof_AudioWorklet\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
