use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaStreamAudioSourceNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode)\n\n*This API requires the following crate features to be activated: `MediaStreamAudioSourceNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaStreamAudioSourceNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaStreamAudioSourceNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaStreamAudioSourceNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(26u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(83u32);
            inform(116u32);
            inform(114u32);
            inform(101u32);
            inform(97u32);
            inform(109u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(83u32);
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for MediaStreamAudioSourceNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaStreamAudioSourceNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaStreamAudioSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaStreamAudioSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaStreamAudioSourceNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaStreamAudioSourceNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaStreamAudioSourceNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaStreamAudioSourceNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaStreamAudioSourceNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaStreamAudioSourceNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaStreamAudioSourceNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaStreamAudioSourceNode {
        #[inline]
        fn from(obj: JsValue) -> MediaStreamAudioSourceNode {
            MediaStreamAudioSourceNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaStreamAudioSourceNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaStreamAudioSourceNode> for MediaStreamAudioSourceNode {
        #[inline]
        fn as_ref(&self) -> &MediaStreamAudioSourceNode {
            self
        }
    }
    impl From<MediaStreamAudioSourceNode> for JsValue {
        #[inline]
        fn from(obj: MediaStreamAudioSourceNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaStreamAudioSourceNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaStreamAudioSourceNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaStreamAudioSourceNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaStreamAudioSourceNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaStreamAudioSourceNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaStreamAudioSourceNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaStreamAudioSourceNode> for AudioNode {
    #[inline]
    fn from(obj: MediaStreamAudioSourceNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for MediaStreamAudioSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStreamAudioSourceNode> for EventTarget {
    #[inline]
    fn from(obj: MediaStreamAudioSourceNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaStreamAudioSourceNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStreamAudioSourceNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaStreamAudioSourceNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaStreamAudioSourceNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "AudioContext",
    feature = "MediaStreamAudioSourceNode",
    feature = "MediaStreamAudioSourceOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaStreamAudioSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <&MediaStreamAudioSourceOptions as WasmDescribe>::describe();
    <MediaStreamAudioSourceNode as WasmDescribe>::describe();
}
impl MediaStreamAudioSourceNode {
    #[cfg(all(
        feature = "AudioContext",
        feature = "MediaStreamAudioSourceNode",
        feature = "MediaStreamAudioSourceOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStreamAudioSourceNode(..)` constructor, creating a new instance of `MediaStreamAudioSourceNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode/MediaStreamAudioSourceNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioSourceNode`, `MediaStreamAudioSourceOptions`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &AudioContext,
        options: &MediaStreamAudioSourceOptions,
    ) -> Result<MediaStreamAudioSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioContext",
            feature = "MediaStreamAudioSourceNode",
            feature = "MediaStreamAudioSourceOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaStreamAudioSourceNode(
                context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options : < & MediaStreamAudioSourceOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaStreamAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaStreamAudioSourceNode(
            context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&MediaStreamAudioSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                let options = < & MediaStreamAudioSourceOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( options ) ;
                __widl_f_new_MediaStreamAudioSourceNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaStreamAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_f7736e9f8ab98ed6: [u8; 278usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD4\0\0\0\0\0\x02\0\0\x02\x1AMediaStreamAudioSourceNode,__widl_instanceof_MediaStreamAudioSourceNode\0\0\0\0'__widl_f_new_MediaStreamAudioSourceNode\x01\0\0\x01\x1AMediaStreamAudioSourceNode\0\x01\x02\x07context\x07options\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
