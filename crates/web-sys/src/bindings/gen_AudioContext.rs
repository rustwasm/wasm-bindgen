use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `AudioContext` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct AudioContext {
    obj: BaseAudioContext,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_AudioContext: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for AudioContext {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(65u32);
            inform(117u32);
            inform(100u32);
            inform(105u32);
            inform(111u32);
            inform(67u32);
            inform(111u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for AudioContext {
        type Target = BaseAudioContext;
        #[inline]
        fn deref(&self) -> &BaseAudioContext {
            &self.obj
        }
    }
    impl IntoWasmAbi for AudioContext {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for AudioContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a AudioContext {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for AudioContext {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            AudioContext {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for AudioContext {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a AudioContext {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for AudioContext {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<AudioContext>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(AudioContext {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for AudioContext {
        #[inline]
        fn from(obj: JsValue) -> AudioContext {
            AudioContext { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for AudioContext {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<AudioContext> for AudioContext {
        #[inline]
        fn as_ref(&self) -> &AudioContext {
            self
        }
    }
    impl From<AudioContext> for JsValue {
        #[inline]
        fn from(obj: AudioContext) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for AudioContext {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_AudioContext(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_AudioContext(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_AudioContext(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            AudioContext { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const AudioContext) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<AudioContext> for BaseAudioContext {
    #[inline]
    fn from(obj: AudioContext) -> BaseAudioContext {
        use wasm_bindgen::JsCast;
        BaseAudioContext::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<BaseAudioContext> for AudioContext {
    #[inline]
    fn as_ref(&self) -> &BaseAudioContext {
        use wasm_bindgen::JsCast;
        BaseAudioContext::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioContext> for EventTarget {
    #[inline]
    fn from(obj: AudioContext) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for AudioContext {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<AudioContext> for ::js_sys::Object {
    #[inline]
    fn from(obj: AudioContext) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for AudioContext {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <AudioContext as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<AudioContext, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_AudioContext(
            ) -> <AudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_AudioContext(
        ) -> <AudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_AudioContext() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioContext as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "AudioContextOptions",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_context_options_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContextOptions as WasmDescribe>::describe();
    <AudioContext as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "AudioContextOptions",))]
    #[allow(bad_style)]
    #[doc = "The `new AudioContext(..)` constructor, creating a new instance of `AudioContext`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/AudioContext)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioContextOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_context_options(
        context_options: &AudioContextOptions,
    ) -> Result<AudioContext, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "AudioContextOptions",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_context_options_AudioContext(
                context_options: <&AudioContextOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_context_options_AudioContext(
            context_options: <&AudioContextOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioContext as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context_options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context_options =
                    <&AudioContextOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        context_options,
                    );
                __widl_f_new_with_context_options_AudioContext(context_options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioContext as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_close_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `close()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/close)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn close(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_close_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_close_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_close_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AudioContext",
    feature = "HtmlMediaElement",
    feature = "MediaElementAudioSourceNode",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_media_element_source_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <&HtmlMediaElement as WasmDescribe>::describe();
    <MediaElementAudioSourceNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(
        feature = "AudioContext",
        feature = "HtmlMediaElement",
        feature = "MediaElementAudioSourceNode",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createMediaElementSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaElementSource)\n\n*This API requires the following crate features to be activated: `AudioContext`, `HtmlMediaElement`, `MediaElementAudioSourceNode`*"]
    #[allow(clippy::all)]
    pub fn create_media_element_source(
        &self,
        media_element: &HtmlMediaElement,
    ) -> Result<MediaElementAudioSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioContext",
            feature = "HtmlMediaElement",
            feature = "MediaElementAudioSourceNode",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_media_element_source_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media_element: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaElementAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_media_element_source_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            media_element: <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaElementAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(media_element);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media_element =
                    <&HtmlMediaElement as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        media_element,
                    );
                __widl_f_create_media_element_source_AudioContext(self_, media_element)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaElementAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioDestinationNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_media_stream_destination_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <MediaStreamAudioDestinationNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioDestinationNode",))]
    #[allow(bad_style)]
    #[doc = "The `createMediaStreamDestination()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamDestination)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*"]
    #[allow(clippy::all)]
    pub fn create_media_stream_destination(
        &self,
    ) -> Result<MediaStreamAudioDestinationNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioDestinationNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_media_stream_destination_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_media_stream_destination_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamAudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_media_stream_destination_AudioContext(self_)
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
    feature = "MediaStream",
    feature = "MediaStreamAudioSourceNode",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_media_stream_source_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <&MediaStream as WasmDescribe>::describe();
    <MediaStreamAudioSourceNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(
        feature = "AudioContext",
        feature = "MediaStream",
        feature = "MediaStreamAudioSourceNode",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createMediaStreamSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createMediaStreamSource)\n\n*This API requires the following crate features to be activated: `AudioContext`, `MediaStream`, `MediaStreamAudioSourceNode`*"]
    #[allow(clippy::all)]
    pub fn create_media_stream_source(
        &self,
        media_stream: &MediaStream,
    ) -> Result<MediaStreamAudioSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioContext",
            feature = "MediaStream",
            feature = "MediaStreamAudioSourceNode",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_media_stream_source_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                media_stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStreamAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_media_stream_source_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            media_stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStreamAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(media_stream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let media_stream =
                    <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(media_stream);
                __widl_f_create_media_stream_source_AudioContext(self_, media_stream)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaStreamAudioSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_suspend_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `suspend()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/suspend)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn suspend(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_suspend_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_suspend_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_suspend_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AnalyserNode", feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_analyser_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <AnalyserNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AnalyserNode", feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createAnalyser()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createAnalyser)\n\n*This API requires the following crate features to be activated: `AnalyserNode`, `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_analyser(&self) -> Result<AnalyserNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AnalyserNode", feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_analyser_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_analyser_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_analyser_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AnalyserNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_biquad_filter_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <BiquadFilterNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `createBiquadFilter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBiquadFilter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn create_biquad_filter(&self) -> Result<BiquadFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_biquad_filter_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_biquad_filter_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_biquad_filter_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBuffer", feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_buffer_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
    <AudioBuffer as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioBuffer", feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBuffer)\n\n*This API requires the following crate features to be activated: `AudioBuffer`, `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_buffer(
        &self,
        number_of_channels: u32,
        length: u32,
        sample_rate: f32,
    ) -> Result<AudioBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBuffer", feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_buffer_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                sample_rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_buffer_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            length: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            sample_rate: <f32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(number_of_channels);
            drop(length);
            drop(sample_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let number_of_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_channels);
                let length = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(length);
                let sample_rate =
                    <f32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(sample_rate);
                __widl_f_create_buffer_AudioContext(self_, number_of_channels, length, sample_rate)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_buffer_source_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <AudioBufferSourceNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `createBufferSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createBufferSource)\n\n*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn create_buffer_source(&self) -> Result<AudioBufferSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioBufferSourceNode", feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_buffer_source_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_buffer_source_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_buffer_source_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioBufferSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ChannelMergerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_merger_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <ChannelMergerNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ChannelMergerNode",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelMerger()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*"]
    #[allow(clippy::all)]
    pub fn create_channel_merger(&self) -> Result<ChannelMergerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ChannelMergerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_merger_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_merger_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_channel_merger_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ChannelMergerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_merger_with_number_of_inputs_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ChannelMergerNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ChannelMergerNode",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelMerger()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelMerger)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelMergerNode`*"]
    #[allow(clippy::all)]
    pub fn create_channel_merger_with_number_of_inputs(
        &self,
        number_of_inputs: u32,
    ) -> Result<ChannelMergerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ChannelMergerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_merger_with_number_of_inputs_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_inputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_merger_with_number_of_inputs_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_inputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(number_of_inputs);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let number_of_inputs =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_inputs);
                __widl_f_create_channel_merger_with_number_of_inputs_AudioContext(
                    self_,
                    number_of_inputs,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelMergerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ChannelSplitterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_splitter_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <ChannelSplitterNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ChannelSplitterNode",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelSplitter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*"]
    #[allow(clippy::all)]
    pub fn create_channel_splitter(&self) -> Result<ChannelSplitterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ChannelSplitterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_splitter_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_splitter_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_channel_splitter_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ChannelSplitterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_channel_splitter_with_number_of_outputs_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ChannelSplitterNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ChannelSplitterNode",))]
    #[allow(bad_style)]
    #[doc = "The `createChannelSplitter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createChannelSplitter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ChannelSplitterNode`*"]
    #[allow(clippy::all)]
    pub fn create_channel_splitter_with_number_of_outputs(
        &self,
        number_of_outputs: u32,
    ) -> Result<ChannelSplitterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ChannelSplitterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_channel_splitter_with_number_of_outputs_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_outputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_channel_splitter_with_number_of_outputs_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_outputs: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(number_of_outputs);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let number_of_outputs =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_outputs);
                __widl_f_create_channel_splitter_with_number_of_outputs_AudioContext(
                    self_,
                    number_of_outputs,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ChannelSplitterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ConstantSourceNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_constant_source_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <ConstantSourceNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ConstantSourceNode",))]
    #[allow(bad_style)]
    #[doc = "The `createConstantSource()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConstantSource)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ConstantSourceNode`*"]
    #[allow(clippy::all)]
    pub fn create_constant_source(&self) -> Result<ConstantSourceNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ConstantSourceNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_constant_source_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_constant_source_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_constant_source_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConstantSourceNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ConvolverNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_convolver_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <ConvolverNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ConvolverNode",))]
    #[allow(bad_style)]
    #[doc = "The `createConvolver()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createConvolver)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ConvolverNode`*"]
    #[allow(clippy::all)]
    pub fn create_convolver(&self) -> Result<ConvolverNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ConvolverNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_convolver_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_convolver_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_convolver_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ConvolverNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "DelayNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_delay_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <DelayNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "DelayNode",))]
    #[allow(bad_style)]
    #[doc = "The `createDelay()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*"]
    #[allow(clippy::all)]
    pub fn create_delay(&self) -> Result<DelayNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "DelayNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_delay_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_delay_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_delay_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DelayNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "DelayNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_delay_with_max_delay_time_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <DelayNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "DelayNode",))]
    #[allow(bad_style)]
    #[doc = "The `createDelay()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDelay)\n\n*This API requires the following crate features to be activated: `AudioContext`, `DelayNode`*"]
    #[allow(clippy::all)]
    pub fn create_delay_with_max_delay_time(
        &self,
        max_delay_time: f64,
    ) -> Result<DelayNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "DelayNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_delay_with_max_delay_time_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_delay_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_delay_with_max_delay_time_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_delay_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DelayNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(max_delay_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_delay_time =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_delay_time);
                __widl_f_create_delay_with_max_delay_time_AudioContext(self_, max_delay_time)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DelayNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "DynamicsCompressorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_dynamics_compressor_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <DynamicsCompressorNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "DynamicsCompressorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createDynamicsCompressor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createDynamicsCompressor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `DynamicsCompressorNode`*"]
    #[allow(clippy::all)]
    pub fn create_dynamics_compressor(
        &self,
    ) -> Result<DynamicsCompressorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "DynamicsCompressorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_dynamics_compressor_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_dynamics_compressor_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_dynamics_compressor_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<DynamicsCompressorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "GainNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_gain_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <GainNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "GainNode",))]
    #[allow(bad_style)]
    #[doc = "The `createGain()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createGain)\n\n*This API requires the following crate features to be activated: `AudioContext`, `GainNode`*"]
    #[allow(clippy::all)]
    pub fn create_gain(&self) -> Result<GainNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "GainNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_gain_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_gain_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_gain_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GainNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "IirFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_iir_filter_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioContext as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <IirFilterNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "IirFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `createIIRFilter()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createIIRFilter)\n\n*This API requires the following crate features to be activated: `AudioContext`, `IirFilterNode`*"]
    #[allow(clippy::all)]
    pub fn create_iir_filter(
        &self,
        feedforward: &::wasm_bindgen::JsValue,
        feedback: &::wasm_bindgen::JsValue,
    ) -> Result<IirFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "IirFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_iir_filter_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                feedforward: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                feedback: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_iir_filter_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            feedforward: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            feedback: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(feedforward);
            drop(feedback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let feedforward =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        feedforward,
                    );
                let feedback =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        feedback,
                    );
                __widl_f_create_iir_filter_AudioContext(self_, feedforward, feedback)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<IirFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_oscillator_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <OscillatorNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createOscillator()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createOscillator)\n\n*This API requires the following crate features to be activated: `AudioContext`, `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn create_oscillator(&self) -> Result<OscillatorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_oscillator_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_oscillator_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_oscillator_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_panner_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <PannerNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `createPanner()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPanner)\n\n*This API requires the following crate features to be activated: `AudioContext`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn create_panner(&self) -> Result<PannerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_panner_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_panner_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_panner_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PannerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "PeriodicWave",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_periodic_wave_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioContext as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <PeriodicWave as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "PeriodicWave",))]
    #[allow(bad_style)]
    #[doc = "The `createPeriodicWave()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`*"]
    #[allow(clippy::all)]
    pub fn create_periodic_wave(
        &self,
        real: &mut [f32],
        imag: &mut [f32],
    ) -> Result<PeriodicWave, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "PeriodicWave",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_periodic_wave_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_periodic_wave_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(real);
            drop(imag);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let real = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(real);
                let imag = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imag);
                __widl_f_create_periodic_wave_AudioContext(self_, real, imag)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AudioContext",
    feature = "PeriodicWave",
    feature = "PeriodicWaveConstraints",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_periodic_wave_with_constraints_AudioContext()
{
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioContext as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&PeriodicWaveConstraints as WasmDescribe>::describe();
    <PeriodicWave as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(
        feature = "AudioContext",
        feature = "PeriodicWave",
        feature = "PeriodicWaveConstraints",
    ))]
    #[allow(bad_style)]
    #[doc = "The `createPeriodicWave()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createPeriodicWave)\n\n*This API requires the following crate features to be activated: `AudioContext`, `PeriodicWave`, `PeriodicWaveConstraints`*"]
    #[allow(clippy::all)]
    pub fn create_periodic_wave_with_constraints(
        &self,
        real: &mut [f32],
        imag: &mut [f32],
        constraints: &PeriodicWaveConstraints,
    ) -> Result<PeriodicWave, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioContext",
            feature = "PeriodicWave",
            feature = "PeriodicWaveConstraints",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_periodic_wave_with_constraints_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                constraints: <&PeriodicWaveConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_periodic_wave_with_constraints_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            real: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            imag: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            constraints: <&PeriodicWaveConstraints as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(real);
            drop(imag);
            drop(constraints);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let real = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(real);
                let imag = <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(imag);
                let constraints =
                    <&PeriodicWaveConstraints as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        constraints,
                    );
                __widl_f_create_periodic_wave_with_constraints_AudioContext(
                    self_,
                    real,
                    imag,
                    constraints,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PeriodicWave as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor(&self) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_script_processor_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_with_buffer_size_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor_with_buffer_size(
        &self,
        buffer_size: u32,
    ) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_with_buffer_size_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_with_buffer_size_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer_size);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer_size =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer_size);
                __widl_f_create_script_processor_with_buffer_size_AudioContext(self_, buffer_size)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels(
        &self,
        buffer_size: u32,
        number_of_input_channels: u32,
    ) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer_size);
            drop(number_of_input_channels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer_size =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer_size);
                let number_of_input_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_input_channels);
                __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_AudioContext ( self_ , buffer_size , number_of_input_channels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioContext as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <ScriptProcessorNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
    #[allow(bad_style)]
    #[doc = "The `createScriptProcessor()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createScriptProcessor)\n\n*This API requires the following crate features to be activated: `AudioContext`, `ScriptProcessorNode`*"]
    #[allow(clippy::all)]
    pub fn create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels(
        &self,
        buffer_size: u32,
        number_of_input_channels: u32,
        number_of_output_channels: u32,
    ) -> Result<ScriptProcessorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "ScriptProcessorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                number_of_output_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            buffer_size: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_input_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            number_of_output_channels: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(buffer_size);
            drop(number_of_input_channels);
            drop(number_of_output_channels);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let buffer_size =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(buffer_size);
                let number_of_input_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(number_of_input_channels);
                let number_of_output_channels =
                    <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        number_of_output_channels,
                    );
                __widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_AudioContext ( self_ , buffer_size , number_of_input_channels , number_of_output_channels )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<ScriptProcessorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "StereoPannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_stereo_panner_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <StereoPannerNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "StereoPannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `createStereoPanner()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createStereoPanner)\n\n*This API requires the following crate features to be activated: `AudioContext`, `StereoPannerNode`*"]
    #[allow(clippy::all)]
    pub fn create_stereo_panner(&self) -> Result<StereoPannerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "StereoPannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_stereo_panner_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <StereoPannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_stereo_panner_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <StereoPannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_stereo_panner_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<StereoPannerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "WaveShaperNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_create_wave_shaper_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <WaveShaperNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "WaveShaperNode",))]
    #[allow(bad_style)]
    #[doc = "The `createWaveShaper()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/createWaveShaper)\n\n*This API requires the following crate features to be activated: `AudioContext`, `WaveShaperNode`*"]
    #[allow(clippy::all)]
    pub fn create_wave_shaper(&self) -> Result<WaveShaperNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "WaveShaperNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_create_wave_shaper_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_create_wave_shaper_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_create_wave_shaper_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<WaveShaperNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_audio_data_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `decodeAudioData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn decode_audio_data(
        &self,
        audio_data: &::js_sys::ArrayBuffer,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_audio_data_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_audio_data_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(audio_data);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let audio_data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        audio_data,
                    );
                __widl_f_decode_audio_data_AudioContext(self_, audio_data)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_audio_data_with_success_callback_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioContext as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `decodeAudioData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn decode_audio_data_with_success_callback(
        &self,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_audio_data_with_success_callback_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_audio_data_with_success_callback_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(audio_data);
            drop(success_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let audio_data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        audio_data,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                __widl_f_decode_audio_data_with_success_callback_AudioContext(
                    self_,
                    audio_data,
                    success_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_decode_audio_data_with_success_callback_and_error_callback_AudioContext(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&AudioContext as WasmDescribe>::describe();
    <&::js_sys::ArrayBuffer as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <&::js_sys::Function as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `decodeAudioData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/decodeAudioData)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn decode_audio_data_with_success_callback_and_error_callback(
        &self,
        audio_data: &::js_sys::ArrayBuffer,
        success_callback: &::js_sys::Function,
        error_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_decode_audio_data_with_success_callback_and_error_callback_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_decode_audio_data_with_success_callback_and_error_callback_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            audio_data: <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            success_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error_callback: <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(audio_data);
            drop(success_callback);
            drop(error_callback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let audio_data =
                    <&::js_sys::ArrayBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        audio_data,
                    );
                let success_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        success_callback,
                    );
                let error_callback =
                    <&::js_sys::Function as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error_callback,
                    );
                __widl_f_decode_audio_data_with_success_callback_and_error_callback_AudioContext(
                    self_,
                    audio_data,
                    success_callback,
                    error_callback,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_resume_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `resume()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/resume)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn resume(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_resume_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_resume_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_resume_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "AudioDestinationNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_destination_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <AudioDestinationNode as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "AudioDestinationNode",))]
    #[allow(bad_style)]
    #[doc = "The `destination` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/destination)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioDestinationNode`*"]
    #[allow(clippy::all)]
    pub fn destination(&self) -> AudioDestinationNode {
        #[cfg(all(feature = "AudioContext", feature = "AudioDestinationNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_destination_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_destination_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_destination_AudioContext(self_)
            };
            <AudioDestinationNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_sample_rate_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <f32 as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `sampleRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/sampleRate)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn sample_rate(&self) -> f32 {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_sample_rate_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_sample_rate_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f32 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_sample_rate_AudioContext(self_)
            };
            <f32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/currentTime)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> f64 {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_time_AudioContext(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "AudioListener",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_listener_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <AudioListener as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "AudioListener",))]
    #[allow(bad_style)]
    #[doc = "The `listener` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/listener)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioListener`*"]
    #[allow(clippy::all)]
    pub fn listener(&self) -> AudioListener {
        #[cfg(all(feature = "AudioContext", feature = "AudioListener",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_listener_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioListener as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_listener_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioListener as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_listener_AudioContext(self_)
            };
            <AudioListener as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "AudioContextState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <AudioContextState as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "AudioContextState",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/state)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioContextState`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> AudioContextState {
        #[cfg(all(feature = "AudioContext", feature = "AudioContextState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioContextState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioContextState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_AudioContext(self_)
            };
            <AudioContextState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContext", feature = "AudioWorklet",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_audio_worklet_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <AudioWorklet as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext", feature = "AudioWorklet",))]
    #[allow(bad_style)]
    #[doc = "The `audioWorklet` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/audioWorklet)\n\n*This API requires the following crate features to be activated: `AudioContext`, `AudioWorklet`*"]
    #[allow(clippy::all)]
    pub fn audio_worklet(&self) -> Result<AudioWorklet, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioContext", feature = "AudioWorklet",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_audio_worklet_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioWorklet as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_audio_worklet_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioWorklet as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_audio_worklet_AudioContext(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<AudioWorklet as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstatechange_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioContext as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn onstatechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstatechange_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstatechange_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstatechange_AudioContext(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioContext",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstatechange_AudioContext() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioContext as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl AudioContext {
    #[cfg(all(feature = "AudioContext",))]
    #[allow(bad_style)]
    #[doc = "The `onstatechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioContext/onstatechange)\n\n*This API requires the following crate features to be activated: `AudioContext`*"]
    #[allow(clippy::all)]
    pub fn set_onstatechange(&self, onstatechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "AudioContext",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstatechange_AudioContext(
                self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstatechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstatechange_AudioContext(
            self_: <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstatechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstatechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&AudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstatechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstatechange,
                    );
                __widl_f_set_onstatechange_AudioContext(self_, onstatechange)
            };
            ()
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_a5e354276c078b6a: [u8; 4730usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}8\x12\0\0\0\0-\0\0\x02\x0CAudioContext\x1E__widl_instanceof_AudioContext\x01\x06webkit\0\0\0\x19__widl_f_new_AudioContext\x01\0\0\x01\x0CAudioContext\0\x01\0\x03new\0\0\0.__widl_f_new_with_context_options_AudioContext\x01\0\0\x01\x0CAudioContext\0\x01\x01\x0Fcontext_options\x03new\0\0\0\x1B__widl_f_close_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x05close\0\0\01__widl_f_create_media_element_source_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\rmedia_element\x18createMediaElementSource\0\0\05__widl_f_create_media_stream_destination_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x1CcreateMediaStreamDestination\0\0\00__widl_f_create_media_stream_source_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\x0Cmedia_stream\x17createMediaStreamSource\0\0\0\x1D__widl_f_suspend_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x07suspend\0\0\0%__widl_f_create_analyser_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x0EcreateAnalyser\0\0\0*__widl_f_create_biquad_filter_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x12createBiquadFilter\0\0\0#__widl_f_create_buffer_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x04\x05self_\x12number_of_channels\x06length\x0Bsample_rate\x0CcreateBuffer\0\0\0*__widl_f_create_buffer_source_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x12createBufferSource\0\0\0+__widl_f_create_channel_merger_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x13createChannelMerger\0\0\0A__widl_f_create_channel_merger_with_number_of_inputs_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\x10number_of_inputs\x13createChannelMerger\0\0\0-__widl_f_create_channel_splitter_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x15createChannelSplitter\0\0\0D__widl_f_create_channel_splitter_with_number_of_outputs_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\x11number_of_outputs\x15createChannelSplitter\0\0\0,__widl_f_create_constant_source_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x14createConstantSource\0\0\0&__widl_f_create_convolver_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x0FcreateConvolver\0\0\0\"__widl_f_create_delay_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x0BcreateDelay\0\0\06__widl_f_create_delay_with_max_delay_time_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\x0Emax_delay_time\x0BcreateDelay\0\0\00__widl_f_create_dynamics_compressor_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x18createDynamicsCompressor\0\0\0!__widl_f_create_gain_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\ncreateGain\0\0\0'__widl_f_create_iir_filter_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x03\x05self_\x0Bfeedforward\x08feedback\x0FcreateIIRFilter\0\0\0'__widl_f_create_oscillator_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x10createOscillator\0\0\0#__widl_f_create_panner_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x0CcreatePanner\0\0\0*__widl_f_create_periodic_wave_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x03\x05self_\x04real\x04imag\x12createPeriodicWave\0\0\0;__widl_f_create_periodic_wave_with_constraints_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x04\x05self_\x04real\x04imag\x0Bconstraints\x12createPeriodicWave\0\0\0-__widl_f_create_script_processor_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x15createScriptProcessor\0\0\0>__widl_f_create_script_processor_with_buffer_size_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\x0Bbuffer_size\x15createScriptProcessor\0\0\0[__widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x03\x05self_\x0Bbuffer_size\x18number_of_input_channels\x15createScriptProcessor\0\0\0y__widl_f_create_script_processor_with_buffer_size_and_number_of_input_channels_and_number_of_output_channels_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x04\x05self_\x0Bbuffer_size\x18number_of_input_channels\x19number_of_output_channels\x15createScriptProcessor\0\0\0*__widl_f_create_stereo_panner_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x12createStereoPanner\0\0\0(__widl_f_create_wave_shaper_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x10createWaveShaper\0\0\0'__widl_f_decode_audio_data_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x02\x05self_\naudio_data\x0FdecodeAudioData\0\0\0=__widl_f_decode_audio_data_with_success_callback_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x03\x05self_\naudio_data\x10success_callback\x0FdecodeAudioData\0\0\0P__widl_f_decode_audio_data_with_success_callback_and_error_callback_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x04\x05self_\naudio_data\x10success_callback\x0Eerror_callback\x0FdecodeAudioData\0\0\0\x1C__widl_f_resume_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\0\x01\x01\x05self_\x06resume\0\0\0!__widl_f_destination_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x01\x0Bdestination\x01\x01\x05self_\x0Bdestination\0\0\0!__widl_f_sample_rate_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x01\nsampleRate\x01\x01\x05self_\nsampleRate\0\0\0\"__widl_f_current_time_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0\x1E__widl_f_listener_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x01\x08listener\x01\x01\x05self_\x08listener\0\0\0\x1B__widl_f_state_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0#__widl_f_audio_worklet_AudioContext\x01\0\0\x01\x0CAudioContext\x01\0\x01\x0CaudioWorklet\x01\x01\x05self_\x0CaudioWorklet\0\0\0#__widl_f_onstatechange_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x01\ronstatechange\x01\x01\x05self_\ronstatechange\0\0\0'__widl_f_set_onstatechange_AudioContext\0\0\0\x01\x0CAudioContext\x01\0\x02\ronstatechange\x01\x02\x05self_\ronstatechange\ronstatechange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
