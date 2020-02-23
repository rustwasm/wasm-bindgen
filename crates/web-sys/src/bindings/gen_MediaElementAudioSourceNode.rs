use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaElementAudioSourceNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode)\n\n*This API requires the following crate features to be activated: `MediaElementAudioSourceNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaElementAudioSourceNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaElementAudioSourceNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaElementAudioSourceNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(27u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(69u32);
            inform(108u32);
            inform(101u32);
            inform(109u32);
            inform(101u32);
            inform(110u32);
            inform(116u32);
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
    impl core::ops::Deref for MediaElementAudioSourceNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaElementAudioSourceNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaElementAudioSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaElementAudioSourceNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaElementAudioSourceNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaElementAudioSourceNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaElementAudioSourceNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaElementAudioSourceNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaElementAudioSourceNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaElementAudioSourceNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaElementAudioSourceNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaElementAudioSourceNode {
        #[inline]
        fn from(obj: JsValue) -> MediaElementAudioSourceNode {
            MediaElementAudioSourceNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaElementAudioSourceNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaElementAudioSourceNode> for MediaElementAudioSourceNode {
        #[inline]
        fn as_ref(&self) -> &MediaElementAudioSourceNode {
            self
        }
    }
    impl From<MediaElementAudioSourceNode> for JsValue {
        #[inline]
        fn from(obj: MediaElementAudioSourceNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaElementAudioSourceNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaElementAudioSourceNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaElementAudioSourceNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaElementAudioSourceNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaElementAudioSourceNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaElementAudioSourceNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaElementAudioSourceNode> for AudioNode {
    #[inline]
    fn from(obj: MediaElementAudioSourceNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for MediaElementAudioSourceNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaElementAudioSourceNode> for EventTarget {
    #[inline]
    fn from(obj: MediaElementAudioSourceNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaElementAudioSourceNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaElementAudioSourceNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaElementAudioSourceNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaElementAudioSourceNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(
    feature = "AudioContext",
    feature = "MediaElementAudioSourceNode",
    feature = "MediaElementAudioSourceOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaElementAudioSourceNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <&MediaElementAudioSourceOptions as WasmDescribe>::describe();
    <MediaElementAudioSourceNode as WasmDescribe>::describe();
}
impl MediaElementAudioSourceNode {
    #[cfg(all(
        feature = "AudioContext",
        feature = "MediaElementAudioSourceNode",
        feature = "MediaElementAudioSourceOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaElementAudioSourceNode(..)` constructor, creating a new instance of `MediaElementAudioSourceNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode/MediaElementAudioSourceNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaElementAudioSourceNode`, `MediaElementAudioSourceOptions`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &AudioContext,
        options: &MediaElementAudioSourceOptions,
    ) -> Result<MediaElementAudioSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioContext",
            feature = "MediaElementAudioSourceNode",
            feature = "MediaElementAudioSourceOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaElementAudioSourceNode(
                context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options : < & MediaElementAudioSourceOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaElementAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaElementAudioSourceNode(
            context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&MediaElementAudioSourceOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaElementAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let options = < & MediaElementAudioSourceOptions as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( options ) ;
                __widl_f_new_MediaElementAudioSourceNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaElementAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_93176fa3410dab77: [u8; 282usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xD8\0\0\0\0\0\x02\0\0\x02\x1BMediaElementAudioSourceNode-__widl_instanceof_MediaElementAudioSourceNode\0\0\0\0(__widl_f_new_MediaElementAudioSourceNode\x01\0\0\x01\x1BMediaElementAudioSourceNode\0\x01\x02\x07context\x07options\x03new\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
