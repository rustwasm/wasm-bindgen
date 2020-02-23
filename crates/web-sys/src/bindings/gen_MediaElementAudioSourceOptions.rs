use super::*;
#[derive(Clone, Debug)]
#[repr(transparent)]
#[allow(clippy::all)]
#[doc = "The `MediaElementAudioSourceOptions` dictionary\n\n\n*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
pub struct MediaElementAudioSourceOptions {
    obj: ::js_sys::Object,
}
#[allow(clippy::all)]
impl MediaElementAudioSourceOptions {
    #[doc = "Construct a new `MediaElementAudioSourceOptions`\n\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn new(media_element: &HtmlMediaElement) -> MediaElementAudioSourceOptions {
        let mut _ret = MediaElementAudioSourceOptions {
            obj: ::js_sys::Object::new(),
        };
        _ret.media_element(media_element);
        return _ret;
    }
    #[cfg(all(feature = "HtmlMediaElement",))]
    #[allow(clippy::all)]
    #[doc = "Configure the `mediaElement` field of this object\n\n\n*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn media_element(&mut self, val: &HtmlMediaElement) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.obj.as_ref(),
            &JsValue::from("mediaElement"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[allow(bad_style)]
#[allow(clippy::all)]
const _CONST_MediaElementAudioSourceOptions: () = {
    use js_sys::Object;
    use wasm_bindgen::__rt::core::mem::ManuallyDrop;
    use wasm_bindgen::convert::*;
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl From<MediaElementAudioSourceOptions> for JsValue {
        #[inline]
        fn from(val: MediaElementAudioSourceOptions) -> JsValue {
            val.obj.into()
        }
    }
    impl AsRef<JsValue> for MediaElementAudioSourceOptions {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl WasmDescribe for MediaElementAudioSourceOptions {
        fn describe() {
            Object::describe();
        }
    }
    impl IntoWasmAbi for MediaElementAudioSourceOptions {
        type Abi = <Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaElementAudioSourceOptions {
        type Abi = <&'a Object as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl FromWasmAbi for MediaElementAudioSourceOptions {
        type Abi = <Object as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(abi: Self::Abi) -> Self {
            MediaElementAudioSourceOptions {
                obj: Object::from_abi(abi),
            }
        }
    }
    impl OptionIntoWasmAbi for MediaElementAudioSourceOptions {
        #[inline]
        fn none() -> Self::Abi {
            Object::none()
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaElementAudioSourceOptions {
        #[inline]
        fn none() -> Self::Abi {
            <&'a Object>::none()
        }
    }
    impl OptionFromWasmAbi for MediaElementAudioSourceOptions {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            Object::is_none(abi)
        }
    }
    impl RefFromWasmAbi for MediaElementAudioSourceOptions {
        type Abi = <Object as RefFromWasmAbi>::Abi;
        type Anchor = ManuallyDrop<MediaElementAudioSourceOptions>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <Object as RefFromWasmAbi>::ref_from_abi(js);
            ManuallyDrop::new(MediaElementAudioSourceOptions {
                obj: ManuallyDrop::into_inner(tmp),
            })
        }
    }
    impl JsCast for MediaElementAudioSourceOptions {
        #[inline]
        fn instanceof(val: &JsValue) -> bool {
            Object::instanceof(val)
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaElementAudioSourceOptions {
                obj: Object::unchecked_from_js(val),
            }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaElementAudioSourceOptions) }
        }
    }
};
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_517ba9459f4bc3be: [u8; 105usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}'\0\0\0\0\0\0\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
