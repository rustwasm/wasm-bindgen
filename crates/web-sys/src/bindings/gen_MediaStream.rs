use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaStream` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaStream {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaStream: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaStream {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(11u32);
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
        }
    }
    impl core::ops::Deref for MediaStream {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaStream {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaStream {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaStream {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaStream {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaStream {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaStream {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaStream {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaStream {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaStream>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaStream {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaStream {
        #[inline]
        fn from(obj: JsValue) -> MediaStream {
            MediaStream { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaStream {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaStream> for MediaStream {
        #[inline]
        fn as_ref(&self) -> &MediaStream {
            self
        }
    }
    impl From<MediaStream> for JsValue {
        #[inline]
        fn from(obj: MediaStream) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaStream {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaStream(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaStream(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaStream(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaStream { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaStream) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaStream> for EventTarget {
    #[inline]
    fn from(obj: MediaStream) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaStream {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaStream> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaStream) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaStream {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <MediaStream as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<MediaStream, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaStream() -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaStream(
        ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_MediaStream() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaStream as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_stream_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <MediaStream as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn new_with_stream(stream: &MediaStream) -> Result<MediaStream, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_stream_MediaStream(
                stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_stream_MediaStream(
            stream: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(stream);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let stream = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(stream);
                __widl_f_new_with_stream_MediaStream(stream)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaStream as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_tracks_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&::wasm_bindgen::JsValue as WasmDescribe>::describe();
    <MediaStream as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaStream(..)` constructor, creating a new instance of `MediaStream`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/MediaStream)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn new_with_tracks(
        tracks: &::wasm_bindgen::JsValue,
    ) -> Result<MediaStream, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_tracks_MediaStream(
                tracks: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_tracks_MediaStream(
            tracks: <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(tracks);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let tracks =
                    <&::wasm_bindgen::JsValue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        tracks,
                    );
                __widl_f_new_with_tracks_MediaStream(tracks)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaStream as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_track_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `addTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/addTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn add_track(&self, track: &MediaStreamTrack) {
        #[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_track_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_track_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(track);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                __widl_f_add_track_MediaStream(self_, track)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clone_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <MediaStream as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `clone()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/clone)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn clone(&self) -> MediaStream {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clone_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clone_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaStream as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clone_MediaStream(self_)
            };
            <MediaStream as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_audio_tracks_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `getAudioTracks()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getAudioTracks)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn get_audio_tracks(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_audio_tracks_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_audio_tracks_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_audio_tracks_MediaStream(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_track_by_id_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStream as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <Option<MediaStreamTrack> as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `getTrackById()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTrackById)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn get_track_by_id(&self, track_id: &str) -> Option<MediaStreamTrack> {
        #[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_track_by_id_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<MediaStreamTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_track_by_id_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track_id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<MediaStreamTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(track_id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track_id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track_id);
                __widl_f_get_track_by_id_MediaStream(self_, track_id)
            };
            <Option<MediaStreamTrack> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_tracks_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `getTracks()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getTracks)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn get_tracks(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_tracks_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_tracks_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_tracks_MediaStream(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_video_tracks_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `getVideoTracks()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/getVideoTracks)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn get_video_tracks(&self) -> ::js_sys::Array {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_video_tracks_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_video_tracks_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_video_tracks_MediaStream(self_)
            };
            <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_track_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStream as WasmDescribe>::describe();
    <&MediaStreamTrack as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
    #[allow(bad_style)]
    #[doc = "The `removeTrack()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/removeTrack)\n\n*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`*"]
    #[allow(clippy::all)]
    pub fn remove_track(&self, track: &MediaStreamTrack) {
        #[cfg(all(feature = "MediaStream", feature = "MediaStreamTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_track_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_track_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            track: <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(track);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let track =
                    <&MediaStreamTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(track);
                __widl_f_remove_track_MediaStream(self_, track)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/id)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_MediaStream(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `active` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/active)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn active(&self) -> bool {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_MediaStream(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onaddtrack_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `onaddtrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn onaddtrack(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onaddtrack_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onaddtrack_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onaddtrack_MediaStream(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onaddtrack_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStream as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `onaddtrack` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onaddtrack)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn set_onaddtrack(&self, onaddtrack: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onaddtrack_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onaddtrack : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onaddtrack_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onaddtrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onaddtrack);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onaddtrack =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onaddtrack,
                    );
                __widl_f_set_onaddtrack_MediaStream(self_, onaddtrack)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onremovetrack_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `onremovetrack` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn onremovetrack(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onremovetrack_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onremovetrack_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onremovetrack_MediaStream(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onremovetrack_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaStream as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `onremovetrack` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/onremovetrack)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn set_onremovetrack(&self, onremovetrack: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onremovetrack_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onremovetrack : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onremovetrack_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onremovetrack: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onremovetrack);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onremovetrack =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onremovetrack,
                    );
                __widl_f_set_onremovetrack_MediaStream(self_, onremovetrack)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaStream",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_MediaStream() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaStream as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl MediaStream {
    #[cfg(all(feature = "MediaStream",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream/currentTime)\n\n*This API requires the following crate features to be activated: `MediaStream`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> f64 {
        #[cfg(all(feature = "MediaStream",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_MediaStream(
                self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_MediaStream(
            self_: <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaStream as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_time_MediaStream(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_5b3d5a33ab6fad6e: [u8; 1497usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x97\x05\0\0\0\0\x12\0\0\x02\x0BMediaStream\x1D__widl_instanceof_MediaStream\0\0\0\0\x18__widl_f_new_MediaStream\x01\0\0\x01\x0BMediaStream\0\x01\0\x03new\0\0\0$__widl_f_new_with_stream_MediaStream\x01\0\0\x01\x0BMediaStream\0\x01\x01\x06stream\x03new\0\0\0$__widl_f_new_with_tracks_MediaStream\x01\0\0\x01\x0BMediaStream\0\x01\x01\x06tracks\x03new\0\0\0\x1E__widl_f_add_track_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x02\x05self_\x05track\x08addTrack\0\0\0\x1A__widl_f_clone_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x01\x05self_\x05clone\0\0\0%__widl_f_get_audio_tracks_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x01\x05self_\x0EgetAudioTracks\0\0\0$__widl_f_get_track_by_id_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x02\x05self_\x08track_id\x0CgetTrackById\0\0\0\x1F__widl_f_get_tracks_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x01\x05self_\tgetTracks\0\0\0%__widl_f_get_video_tracks_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x01\x05self_\x0EgetVideoTracks\0\0\0!__widl_f_remove_track_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\0\x01\x02\x05self_\x05track\x0BremoveTrack\0\0\0\x17__widl_f_id_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x1B__widl_f_active_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x01\x06active\x01\x01\x05self_\x06active\0\0\0\x1F__widl_f_onaddtrack_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x01\nonaddtrack\x01\x01\x05self_\nonaddtrack\0\0\0#__widl_f_set_onaddtrack_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x02\nonaddtrack\x01\x02\x05self_\nonaddtrack\nonaddtrack\0\0\0\"__widl_f_onremovetrack_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x01\ronremovetrack\x01\x01\x05self_\ronremovetrack\0\0\0&__widl_f_set_onremovetrack_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x02\ronremovetrack\x01\x02\x05self_\ronremovetrack\ronremovetrack\0\0\0!__widl_f_current_time_MediaStream\0\0\0\x01\x0BMediaStream\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
