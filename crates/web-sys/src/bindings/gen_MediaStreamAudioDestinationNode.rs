use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaStreamAudioDestinationNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode)\n\n*This API requires the following crate features to be activated: `MediaStreamAudioDestinationNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaStreamAudioDestinationNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaStreamAudioDestinationNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaStreamAudioDestinationNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(31u32);
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
            inform(68u32);
            inform(101u32);
            inform(115u32);
            inform(116u32);
            inform(105u32);
            inform(110u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for MediaStreamAudioDestinationNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaStreamAudioDestinationNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaStreamAudioDestinationNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaStreamAudioDestinationNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaStreamAudioDestinationNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaStreamAudioDestinationNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaStreamAudioDestinationNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaStreamAudioDestinationNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaStreamAudioDestinationNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaStreamAudioDestinationNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaStreamAudioDestinationNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaStreamAudioDestinationNode {
        #[inline]
        fn from(obj: JsValue) -> MediaStreamAudioDestinationNode {
            MediaStreamAudioDestinationNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaStreamAudioDestinationNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaStreamAudioDestinationNode> for MediaStreamAudioDestinationNode {
        #[inline]
        fn as_ref(&self) -> &MediaStreamAudioDestinationNode {
            self
        }
    }
    impl From<MediaStreamAudioDestinationNode> for JsValue {
        #[inline]
        fn from(obj: MediaStreamAudioDestinationNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaStreamAudioDestinationNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaStreamAudioDestinationNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaStreamAudioDestinationNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaStreamAudioDestinationNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaStreamAudioDestinationNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaStreamAudioDestinationNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaStreamAudioDestinationNode> for AudioNode {
    #[inline]
    fn from(obj: MediaStreamAudioDestinationNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for MediaStreamAudioDestinationNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStreamAudioDestinationNode> for EventTarget {
    #[inline]
    fn from(obj: MediaStreamAudioDestinationNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaStreamAudioDestinationNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStreamAudioDestinationNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaStreamAudioDestinationNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaStreamAudioDestinationNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioDestinationNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaStreamAudioDestinationNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <MediaStreamAudioDestinationNode as WasmDescribe>::describe();
}
impl MediaStreamAudioDestinationNode {
    #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioDestinationNode",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*"]
    #[allow(clippy::all)]
    pub fn new(
        context: &AudioContext,
    ) -> Result<MediaStreamAudioDestinationNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioDestinationNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaStreamAudioDestinationNode(
                context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaStreamAudioDestinationNode(
            context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                __widl_f_new_MediaStreamAudioDestinationNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[cfg(all(
    feature = "AudioContext",
    feature = "AudioNodeOptions",
    feature = "MediaStreamAudioDestinationNode",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_MediaStreamAudioDestinationNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <&AudioNodeOptions as WasmDescribe>::describe();
    <MediaStreamAudioDestinationNode as WasmDescribe>::describe();
}
impl MediaStreamAudioDestinationNode {
    #[cfg(all(
        feature = "AudioContext",
        feature = "AudioNodeOptions",
        feature = "MediaStreamAudioDestinationNode",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioNodeOptions`, `MediaStreamAudioDestinationNode`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &AudioContext,
        options: &AudioNodeOptions,
    ) -> Result<MediaStreamAudioDestinationNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioContext",
            feature = "AudioNodeOptions",
            feature = "MediaStreamAudioDestinationNode",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_MediaStreamAudioDestinationNode(
                context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&AudioNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_MediaStreamAudioDestinationNode(
            context: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&AudioNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                let options =
                    <&AudioNodeOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_MediaStreamAudioDestinationNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(
                <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    _ret,
                ),
            )
        }
    }
}
#[cfg(all(feature = "MediaStream", feature = "MediaStreamAudioDestinationNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stream_MediaStreamAudioDestinationNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStreamAudioDestinationNode as WasmDescribe>::describe();
    <MediaStream as WasmDescribe>::describe();
}
impl MediaStreamAudioDestinationNode {
    #[cfg(all(feature = "MediaStream", feature = "MediaStreamAudioDestinationNode",))]
    #[allow(bad_style)]
    #[doc = "The `stream` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/stream)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioDestinationNode`*"]
    #[allow(clippy::all)]
    pub fn stream(&self) -> MediaStream {
        #[cfg(all(feature = "MediaStream", feature = "MediaStreamAudioDestinationNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stream_MediaStreamAudioDestinationNode(
                self_ : < & MediaStreamAudioDestinationNode as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stream_MediaStreamAudioDestinationNode(
            self_: <&MediaStreamAudioDestinationNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = < & MediaStreamAudioDestinationNode as wasm_bindgen :: convert :: IntoWasmAbi > :: into_abi ( self ) ;
                __widl_f_stream_MediaStreamAudioDestinationNode(self_)
            };
            <MediaStream as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_68c1c74c30d4bdd4: [u8; 522usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xC8\x01\0\0\0\0\x04\0\0\x02\x1FMediaStreamAudioDestinationNode1__widl_instanceof_MediaStreamAudioDestinationNode\0\0\0\0,__widl_f_new_MediaStreamAudioDestinationNode\x01\0\0\x01\x1FMediaStreamAudioDestinationNode\0\x01\x01\x07context\x03new\0\0\09__widl_f_new_with_options_MediaStreamAudioDestinationNode\x01\0\0\x01\x1FMediaStreamAudioDestinationNode\0\x01\x02\x07context\x07options\x03new\0\0\0/__widl_f_stream_MediaStreamAudioDestinationNode\0\0\0\x01\x1FMediaStreamAudioDestinationNode\x01\0\x01\x06stream\x01\x01\x05self_\x06stream\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
