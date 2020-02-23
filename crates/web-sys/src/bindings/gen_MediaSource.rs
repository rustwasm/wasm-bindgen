use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `MediaSource` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct MediaSource {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_MediaSource: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for MediaSource {
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
            inform(111u32);
            inform(117u32);
            inform(114u32);
            inform(99u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for MediaSource {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for MediaSource {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for MediaSource {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a MediaSource {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for MediaSource {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            MediaSource {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for MediaSource {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a MediaSource {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for MediaSource {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<MediaSource>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(MediaSource {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for MediaSource {
        #[inline]
        fn from(obj: JsValue) -> MediaSource {
            MediaSource { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for MediaSource {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<MediaSource> for MediaSource {
        #[inline]
        fn as_ref(&self) -> &MediaSource {
            self
        }
    }
    impl From<MediaSource> for JsValue {
        #[inline]
        fn from(obj: MediaSource) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for MediaSource {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_MediaSource(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_MediaSource(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_MediaSource(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            MediaSource { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const MediaSource) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<MediaSource> for EventTarget {
    #[inline]
    fn from(obj: MediaSource) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for MediaSource {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<MediaSource> for ::js_sys::Object {
    #[inline]
    fn from(obj: MediaSource) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for MediaSource {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <MediaSource as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `new MediaSource(..)` constructor, creating a new instance of `MediaSource`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/MediaSource)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<MediaSource, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_MediaSource() -> <MediaSource as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_MediaSource(
        ) -> <MediaSource as wasm_bindgen::convert::FromWasmAbi>::Abi {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_MediaSource() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<MediaSource as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_source_buffer_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <SourceBuffer as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource", feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `addSourceBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/addSourceBuffer)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn add_source_buffer(&self, type_: &str) -> Result<SourceBuffer, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource", feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_source_buffer_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SourceBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_source_buffer_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SourceBuffer as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_add_source_buffer_MediaSource(self_, type_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<SourceBuffer as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_clear_live_seekable_range_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `clearLiveSeekableRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/clearLiveSeekableRange)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn clear_live_seekable_range(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_clear_live_seekable_range_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_clear_live_seekable_range_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_clear_live_seekable_range_MediaSource(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_of_stream_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `endOfStream()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn end_of_stream(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_of_stream_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_of_stream_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_end_of_stream_MediaSource(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "MediaSourceEndOfStreamError",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_of_stream_with_error_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <MediaSourceEndOfStreamError as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource", feature = "MediaSourceEndOfStreamError",))]
    #[allow(bad_style)]
    #[doc = "The `endOfStream()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/endOfStream)\n\n*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceEndOfStreamError`*"]
    #[allow(clippy::all)]
    pub fn end_of_stream_with_error(
        &self,
        error: MediaSourceEndOfStreamError,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource", feature = "MediaSourceEndOfStreamError",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_of_stream_with_error_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                error: <MediaSourceEndOfStreamError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_of_stream_with_error_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            error: <MediaSourceEndOfStreamError as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(error);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let error =
                    <MediaSourceEndOfStreamError as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        error,
                    );
                __widl_f_end_of_stream_with_error_MediaSource(self_, error)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_is_type_supported_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&str as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `isTypeSupported()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/isTypeSupported)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn is_type_supported(type_: &str) -> bool {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_is_type_supported_MediaSource(
                type_: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_is_type_supported_MediaSource(
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
                __widl_f_is_type_supported_MediaSource(type_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "SourceBuffer",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_source_buffer_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <&SourceBuffer as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource", feature = "SourceBuffer",))]
    #[allow(bad_style)]
    #[doc = "The `removeSourceBuffer()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/removeSourceBuffer)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBuffer`*"]
    #[allow(clippy::all)]
    pub fn remove_source_buffer(
        &self,
        source_buffer: &SourceBuffer,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource", feature = "SourceBuffer",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_source_buffer_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                source_buffer: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_source_buffer_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            source_buffer: <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(source_buffer);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let source_buffer =
                    <&SourceBuffer as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source_buffer);
                __widl_f_remove_source_buffer_MediaSource(self_, source_buffer)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_live_seekable_range_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <&MediaSource as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `setLiveSeekableRange()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/setLiveSeekableRange)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn set_live_seekable_range(
        &self,
        start: f64,
        end: f64,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_live_seekable_range_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_live_seekable_range_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start);
            drop(end);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start);
                let end = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end);
                __widl_f_set_live_seekable_range_MediaSource(self_, start, end)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_source_buffers_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <SourceBufferList as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource", feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `sourceBuffers` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/sourceBuffers)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn source_buffers(&self) -> SourceBufferList {
        #[cfg(all(feature = "MediaSource", feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_source_buffers_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SourceBufferList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_source_buffers_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SourceBufferList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_source_buffers_MediaSource(self_)
            };
            <SourceBufferList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "SourceBufferList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_source_buffers_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <SourceBufferList as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource", feature = "SourceBufferList",))]
    #[allow(bad_style)]
    #[doc = "The `activeSourceBuffers` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/activeSourceBuffers)\n\n*This API requires the following crate features to be activated: `MediaSource`, `SourceBufferList`*"]
    #[allow(clippy::all)]
    pub fn active_source_buffers(&self) -> SourceBufferList {
        #[cfg(all(feature = "MediaSource", feature = "SourceBufferList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_source_buffers_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <SourceBufferList as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_source_buffers_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <SourceBufferList as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_source_buffers_MediaSource(self_)
            };
            <SourceBufferList as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource", feature = "MediaSourceReadyState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_state_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <MediaSourceReadyState as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource", feature = "MediaSourceReadyState",))]
    #[allow(bad_style)]
    #[doc = "The `readyState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/readyState)\n\n*This API requires the following crate features to be activated: `MediaSource`, `MediaSourceReadyState`*"]
    #[allow(clippy::all)]
    pub fn ready_state(&self) -> MediaSourceReadyState {
        #[cfg(all(feature = "MediaSource", feature = "MediaSourceReadyState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_state_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <MediaSourceReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_state_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <MediaSourceReadyState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_state_MediaSource(self_)
            };
            <MediaSourceReadyState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_duration_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `duration` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn duration(&self) -> f64 {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_duration_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_duration_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_duration_MediaSource(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_duration_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `duration` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/duration)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn set_duration(&self, duration: f64) {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_duration_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_duration_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            duration: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(duration);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let duration = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(duration);
                __widl_f_set_duration_MediaSource(self_, duration)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsourceopen_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `onsourceopen` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn onsourceopen(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsourceopen_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsourceopen_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsourceopen_MediaSource(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsourceopen_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `onsourceopen` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceopen)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn set_onsourceopen(&self, onsourceopen: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsourceopen_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsourceopen : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsourceopen_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsourceopen: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsourceopen);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsourceopen =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsourceopen,
                    );
                __widl_f_set_onsourceopen_MediaSource(self_, onsourceopen)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsourceended_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `onsourceended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn onsourceended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsourceended_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsourceended_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsourceended_MediaSource(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsourceended_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `onsourceended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceended)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn set_onsourceended(&self, onsourceended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsourceended_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsourceended : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsourceended_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsourceended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onsourceended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsourceended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsourceended,
                    );
                __widl_f_set_onsourceended_MediaSource(self_, onsourceended)
            };
            ()
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onsourceclosed_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&MediaSource as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `onsourceclosed` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclosed)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn onsourceclosed(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onsourceclosed_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onsourceclosed_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onsourceclosed_MediaSource(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "MediaSource",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onsourceclosed_MediaSource() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&MediaSource as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl MediaSource {
    #[cfg(all(feature = "MediaSource",))]
    #[allow(bad_style)]
    #[doc = "The `onsourceclosed` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaSource/onsourceclosed)\n\n*This API requires the following crate features to be activated: `MediaSource`*"]
    #[allow(clippy::all)]
    pub fn set_onsourceclosed(&self, onsourceclosed: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "MediaSource",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onsourceclosed_MediaSource(
                self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onsourceclosed : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onsourceclosed_MediaSource(
            self_: <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onsourceclosed : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onsourceclosed);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&MediaSource as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onsourceclosed =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onsourceclosed,
                    );
                __widl_f_set_onsourceclosed_MediaSource(self_, onsourceclosed)
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
pub static __WASM_BINDGEN_GENERATED_11888604f4dd1c16: [u8; 1933usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}K\x07\0\0\0\0\x14\0\0\x02\x0BMediaSource\x1D__widl_instanceof_MediaSource\0\0\0\0\x18__widl_f_new_MediaSource\x01\0\0\x01\x0BMediaSource\0\x01\0\x03new\0\0\0&__widl_f_add_source_buffer_MediaSource\x01\0\0\x01\x0BMediaSource\x01\0\0\x01\x02\x05self_\x05type_\x0FaddSourceBuffer\0\0\0.__widl_f_clear_live_seekable_range_MediaSource\x01\0\0\x01\x0BMediaSource\x01\0\0\x01\x01\x05self_\x16clearLiveSeekableRange\0\0\0\"__widl_f_end_of_stream_MediaSource\x01\0\0\x01\x0BMediaSource\x01\0\0\x01\x01\x05self_\x0BendOfStream\0\0\0-__widl_f_end_of_stream_with_error_MediaSource\x01\0\0\x01\x0BMediaSource\x01\0\0\x01\x02\x05self_\x05error\x0BendOfStream\0\0\0&__widl_f_is_type_supported_MediaSource\0\0\0\x01\x0BMediaSource\x01\x01\0\x01\x01\x05type_\x0FisTypeSupported\0\0\0)__widl_f_remove_source_buffer_MediaSource\x01\0\0\x01\x0BMediaSource\x01\0\0\x01\x02\x05self_\rsource_buffer\x12removeSourceBuffer\0\0\0,__widl_f_set_live_seekable_range_MediaSource\x01\0\0\x01\x0BMediaSource\x01\0\0\x01\x03\x05self_\x05start\x03end\x14setLiveSeekableRange\0\0\0#__widl_f_source_buffers_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\rsourceBuffers\x01\x01\x05self_\rsourceBuffers\0\0\0*__widl_f_active_source_buffers_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\x13activeSourceBuffers\x01\x01\x05self_\x13activeSourceBuffers\0\0\0 __widl_f_ready_state_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\nreadyState\x01\x01\x05self_\nreadyState\0\0\0\x1D__widl_f_duration_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\x08duration\x01\x01\x05self_\x08duration\0\0\0!__widl_f_set_duration_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x02\x08duration\x01\x02\x05self_\x08duration\x08duration\0\0\0!__widl_f_onsourceopen_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\x0Consourceopen\x01\x01\x05self_\x0Consourceopen\0\0\0%__widl_f_set_onsourceopen_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x02\x0Consourceopen\x01\x02\x05self_\x0Consourceopen\x0Consourceopen\0\0\0\"__widl_f_onsourceended_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\ronsourceended\x01\x01\x05self_\ronsourceended\0\0\0&__widl_f_set_onsourceended_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x02\ronsourceended\x01\x02\x05self_\ronsourceended\ronsourceended\0\0\0#__widl_f_onsourceclosed_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x01\x0Eonsourceclosed\x01\x01\x05self_\x0Eonsourceclosed\0\0\0'__widl_f_set_onsourceclosed_MediaSource\0\0\0\x01\x0BMediaSource\x01\0\x02\x0Eonsourceclosed\x01\x02\x05self_\x0Eonsourceclosed\x0Eonsourceclosed\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
