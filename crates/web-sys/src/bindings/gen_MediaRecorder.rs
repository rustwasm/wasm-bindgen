use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaRecorder` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaRecorder {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaRecorder: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaRecorder {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(13u32);
            inform(77u32);
            inform(101u32);
            inform(100u32);
            inform(105u32);
            inform(97u32);
            inform(82u32);
            inform(101u32);
            inform(99u32);
            inform(111u32);
            inform(114u32);
            inform(100u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for MediaRecorder {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaRecorder {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaRecorder {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaRecorder {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaRecorder {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaRecorder {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaRecorder {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaRecorder {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaRecorder {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaRecorder>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaRecorder {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaRecorder {
        #[inline]
        fn from(obj: JsValue) -> MediaRecorder {
            MediaRecorder { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaRecorder {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaRecorder> for MediaRecorder {
        #[inline]
        fn as_ref(&self) -> &MediaRecorder {
            self
        }
    }
    impl From<MediaRecorder> for JsValue {
        #[inline]
        fn from(obj: MediaRecorder) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaRecorder {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaRecorder(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaRecorder(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaRecorder(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaRecorder { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaRecorder) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaRecorder> for EventTarget {
    #[inline]
    fn from(obj: MediaRecorder) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaRecorder {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaRecorder> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaRecorder) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaRecorder {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaRecorder", feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_media_stream_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <MediaRecorder as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder", feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn new_with_media_stream(
        stream: &MediaStream,
    ) -> Result<MediaRecorder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder", feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_media_stream_MediaRecorder(
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_media_stream_MediaRecorder(
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(stream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                __widl_f_new_with_media_stream_MediaRecorder(stream)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "MediaRecorder",
    feature = "MediaRecorderOptions",
    feature = "MediaStream",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_media_stream_and_media_recorder_options_MediaRecorder(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStream as WasmDescribe>::describe();
    <&MediaRecorderOptions as WasmDescribe>::describe();
    <MediaRecorder as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(
        feature = "MediaRecorder",
        feature = "MediaRecorderOptions",
        feature = "MediaStream",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `MediaRecorderOptions`, `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn new_with_media_stream_and_media_recorder_options(
        stream: &MediaStream,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "MediaRecorder",
            feature = "MediaRecorderOptions",
            feature = "MediaStream",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_media_stream_and_media_recorder_options_MediaRecorder(
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&MediaRecorderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_media_stream_and_media_recorder_options_MediaRecorder(
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&MediaRecorderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(stream);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                let options =
                    <&MediaRecorderOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_media_stream_and_media_recorder_options_MediaRecorder(
                    stream, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_audio_node_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&AudioNode as WasmDescribe>::describe();
    <MediaRecorder as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "AudioNode", feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn new_with_audio_node(node: &AudioNode) -> Result<MediaRecorder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode", feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_audio_node_MediaRecorder(
                node: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_audio_node_MediaRecorder(
            node: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(node);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let node = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                __widl_f_new_with_audio_node_MediaRecorder(node)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "AudioNode", feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_audio_node_and_u32_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <MediaRecorder as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "AudioNode", feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn new_with_audio_node_and_u32(
        node: &AudioNode,
        output: u32,
    ) -> Result<MediaRecorder, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "AudioNode", feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_audio_node_and_u32_MediaRecorder(
                node: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_audio_node_and_u32_MediaRecorder(
            node: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(node);
            drop(output);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let node = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                __widl_f_new_with_audio_node_and_u32_MediaRecorder(node, output)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "AudioNode",
    feature = "MediaRecorder",
    feature = "MediaRecorderOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_audio_node_and_u32_and_options_MediaRecorder(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&AudioNode as WasmDescribe>::describe();
    <u32 as WasmDescribe>::describe();
    <&MediaRecorderOptions as WasmDescribe>::describe();
    <MediaRecorder as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(
        feature = "AudioNode",
        feature = "MediaRecorder",
        feature = "MediaRecorderOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new MediaRecorder(..)` constructor, creating a new instance of `MediaRecorder`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/MediaRecorder)\n\n*This API requires the following crate features to be activated: `AudioNode`, `MediaRecorder`, `MediaRecorderOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_audio_node_and_u32_and_options(
        node: &AudioNode,
        output: u32,
        options: &MediaRecorderOptions,
    ) -> Result<MediaRecorder, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "AudioNode",
            feature = "MediaRecorder",
            feature = "MediaRecorderOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_audio_node_and_u32_and_options_MediaRecorder(
                node: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&MediaRecorderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_audio_node_and_u32_and_options_MediaRecorder(
            node: <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            output: <u32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&MediaRecorderOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(node);
            drop(output);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let node = <&AudioNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(node);
                let output = <u32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(output);
                let options =
                    <&MediaRecorderOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_audio_node_and_u32_and_options_MediaRecorder(
                    node, output, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaRecorder as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_type_supported_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `isTypeSupported()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/isTypeSupported)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn is_type_supported(type_: &str) -> bool {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_type_supported_MediaRecorder(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_type_supported_MediaRecorder(
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_is_type_supported_MediaRecorder(type_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pause_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `pause()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/pause)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn pause(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pause_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pause_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pause_MediaRecorder(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_request_data_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `requestData()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/requestData)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn request_data(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_request_data_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_request_data_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_request_data_MediaRecorder(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_resume_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `resume()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/resume)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn resume(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_resume_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_resume_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_resume_MediaRecorder(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_MediaRecorder(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_time_slice_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <i32 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/start)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn start_with_time_slice(&self, time_slice: i32) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_time_slice_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                time_slice: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_time_slice_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            time_slice: <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(time_slice);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let time_slice = <i32 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(time_slice);
                __widl_f_start_with_time_slice_MediaRecorder(self_, time_slice)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stop)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_MediaRecorder(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaRecorder", feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stream_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <MediaStream as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder", feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `stream` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/stream)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn stream(&self) -> MediaStream {
        #[cfg(all(feature = "MediaRecorder", feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stream_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stream_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stream_MediaRecorder(self_)
            };
            <MediaStream as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder", feature = "RecordingState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_state_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <RecordingState as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder", feature = "RecordingState",))]
    #[allow(bad_style)]
    #[doc = "The `state` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/state)\n\n*This API requires the following crate features to be activated: `MediaRecorder`, `RecordingState`*"]
    #[allow(clippy::all)]
    pub fn state(&self) -> RecordingState {
        #[cfg(all(feature = "MediaRecorder", feature = "RecordingState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_state_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <RecordingState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_state_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <RecordingState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_state_MediaRecorder(self_)
            };
            <RecordingState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mime_type_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `mimeType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/mimeType)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn mime_type(&self) -> String {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mime_type_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mime_type_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mime_type_MediaRecorder(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondataavailable_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `ondataavailable` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn ondataavailable(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondataavailable_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondataavailable_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondataavailable_MediaRecorder(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondataavailable_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `ondataavailable` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/ondataavailable)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn set_ondataavailable(&self, ondataavailable: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondataavailable_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondataavailable : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondataavailable_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondataavailable : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ondataavailable);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondataavailable =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondataavailable,
                    );
                __widl_f_set_ondataavailable_MediaRecorder(self_, ondataavailable)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onerror_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn onerror(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onerror_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onerror_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onerror_MediaRecorder(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onerror_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onerror` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onerror)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn set_onerror(&self, onerror: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onerror_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onerror_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onerror: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onerror);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onerror =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onerror,
                    );
                __widl_f_set_onerror_MediaRecorder(self_, onerror)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstart_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onstart` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn onstart(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstart_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstart_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstart_MediaRecorder(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstart_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onstart` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstart)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn set_onstart(&self, onstart: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstart_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstart_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstart: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstart);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstart =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstart,
                    );
                __widl_f_set_onstart_MediaRecorder(self_, onstart)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onstop_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onstop` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn onstop(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onstop_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onstop_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onstop_MediaRecorder(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onstop_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onstop` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onstop)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn set_onstop(&self, onstop: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onstop_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onstop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onstop_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onstop: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onstop);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onstop =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onstop,
                    );
                __widl_f_set_onstop_MediaRecorder(self_, onstop)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onwarning_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onwarning` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn onwarning(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onwarning_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onwarning_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onwarning_MediaRecorder(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaRecorder",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onwarning_MediaRecorder() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaRecorder as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaRecorder {
    #[cfg(all(feature = "MediaRecorder",))]
    #[allow(bad_style)]
    #[doc = "The `onwarning` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorder/onwarning)\n\n*This API requires the following crate features to be activated: `MediaRecorder`*"]
    #[allow(clippy::all)]
    pub fn set_onwarning(&self, onwarning: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaRecorder",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onwarning_MediaRecorder(
                self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onwarning: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onwarning_MediaRecorder(
            self_: <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onwarning: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onwarning);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaRecorder as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onwarning =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onwarning,
                    );
                __widl_f_set_onwarning_MediaRecorder(self_, onwarning)
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
pub static __WASM_BINDGEN_GENERATED_1debfa579db7025c: [u8; 2304usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xBE\x08\0\0\0\0\x1A\0\0\x02\rMediaRecorder\x1F__widl_instanceof_MediaRecorder\0\0\0\0,__widl_f_new_with_media_stream_MediaRecorder\x01\0\0\x01\rMediaRecorder\0\x01\x01\x06stream\x03new\0\0\0G__widl_f_new_with_media_stream_and_media_recorder_options_MediaRecorder\x01\0\0\x01\rMediaRecorder\0\x01\x02\x06stream\x07options\x03new\0\0\0*__widl_f_new_with_audio_node_MediaRecorder\x01\0\0\x01\rMediaRecorder\0\x01\x01\x04node\x03new\0\0\02__widl_f_new_with_audio_node_and_u32_MediaRecorder\x01\0\0\x01\rMediaRecorder\0\x01\x02\x04node\x06output\x03new\0\0\0>__widl_f_new_with_audio_node_and_u32_and_options_MediaRecorder\x01\0\0\x01\rMediaRecorder\0\x01\x03\x04node\x06output\x07options\x03new\0\0\0(__widl_f_is_type_supported_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\x01\0\x01\x01\x05type_\x0FisTypeSupported\0\0\0\x1C__widl_f_pause_MediaRecorder\x01\0\0\x01\rMediaRecorder\x01\0\0\x01\x01\x05self_\x05pause\0\0\0#__widl_f_request_data_MediaRecorder\x01\0\0\x01\rMediaRecorder\x01\0\0\x01\x01\x05self_\x0BrequestData\0\0\0\x1D__widl_f_resume_MediaRecorder\x01\0\0\x01\rMediaRecorder\x01\0\0\x01\x01\x05self_\x06resume\0\0\0\x1C__widl_f_start_MediaRecorder\x01\0\0\x01\rMediaRecorder\x01\0\0\x01\x01\x05self_\x05start\0\0\0,__widl_f_start_with_time_slice_MediaRecorder\x01\0\0\x01\rMediaRecorder\x01\0\0\x01\x02\x05self_\ntime_slice\x05start\0\0\0\x1B__widl_f_stop_MediaRecorder\x01\0\0\x01\rMediaRecorder\x01\0\0\x01\x01\x05self_\x04stop\0\0\0\x1D__widl_f_stream_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x06stream\x01\x01\x05self_\x06stream\0\0\0\x1C__widl_f_state_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x05state\x01\x01\x05self_\x05state\0\0\0 __widl_f_mime_type_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x08mimeType\x01\x01\x05self_\x08mimeType\0\0\0&__widl_f_ondataavailable_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x0Fondataavailable\x01\x01\x05self_\x0Fondataavailable\0\0\0*__widl_f_set_ondataavailable_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x02\x0Fondataavailable\x01\x02\x05self_\x0Fondataavailable\x0Fondataavailable\0\0\0\x1E__widl_f_onerror_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x07onerror\x01\x01\x05self_\x07onerror\0\0\0\"__widl_f_set_onerror_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x02\x07onerror\x01\x02\x05self_\x07onerror\x07onerror\0\0\0\x1E__widl_f_onstart_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x07onstart\x01\x01\x05self_\x07onstart\0\0\0\"__widl_f_set_onstart_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x02\x07onstart\x01\x02\x05self_\x07onstart\x07onstart\0\0\0\x1D__widl_f_onstop_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\x06onstop\x01\x01\x05self_\x06onstop\0\0\0!__widl_f_set_onstop_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x02\x06onstop\x01\x02\x05self_\x06onstop\x06onstop\0\0\0 __widl_f_onwarning_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x01\tonwarning\x01\x01\x05self_\tonwarning\0\0\0$__widl_f_set_onwarning_MediaRecorder\0\0\0\x01\rMediaRecorder\x01\0\x02\tonwarning\x01\x02\x05self_\tonwarning\tonwarning\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
