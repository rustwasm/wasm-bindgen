use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TextTrack` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TextTrack {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TextTrack: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TextTrack {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
        }
    }
    impl core::ops::Deref for TextTrack {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for TextTrack {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TextTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TextTrack {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TextTrack {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TextTrack {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TextTrack {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TextTrack {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TextTrack {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TextTrack>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TextTrack {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TextTrack {
        #[inline]
        fn from(obj: JsValue) -> TextTrack {
            TextTrack { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TextTrack {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TextTrack> for TextTrack {
        #[inline]
        fn as_ref(&self) -> &TextTrack {
            self
        }
    }
    impl From<TextTrack> for JsValue {
        #[inline]
        fn from(obj: TextTrack) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TextTrack {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TextTrack(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TextTrack(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TextTrack(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TextTrack { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TextTrack) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TextTrack> for EventTarget {
    #[inline]
    fn from(obj: TextTrack) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for TextTrack {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TextTrack> for ::js_sys::Object {
    #[inline]
    fn from(obj: TextTrack) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TextTrack {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TextTrack", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_add_cue_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrack as WasmDescribe>::describe();
    <&VttCue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `addCue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/addCue)\n\n*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn add_cue(&self, cue: &VttCue) {
        #[cfg(all(feature = "TextTrack", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_add_cue_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cue: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_add_cue_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cue: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cue);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cue = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cue);
                __widl_f_add_cue_TextTrack(self_, cue)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "VttCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_remove_cue_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrack as WasmDescribe>::describe();
    <&VttCue as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "VttCue",))]
    #[allow(bad_style)]
    #[doc = "The `removeCue()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/removeCue)\n\n*This API requires the following crate features to be activated: `TextTrack`, `VttCue`*"]
    #[allow(clippy::all)]
    pub fn remove_cue(&self, cue: &VttCue) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "TextTrack", feature = "VttCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_remove_cue_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cue: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_remove_cue_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cue: <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cue);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cue = <&VttCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cue);
                __widl_f_remove_cue_TextTrack(self_, cue)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_kind_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <TextTrackKind as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
    #[allow(bad_style)]
    #[doc = "The `kind` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/kind)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackKind`*"]
    #[allow(clippy::all)]
    pub fn kind(&self) -> TextTrackKind {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackKind",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_kind_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextTrackKind as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_kind_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextTrackKind as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_kind_TextTrack(self_)
            };
            <TextTrackKind as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_label_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `label` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/label)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn label(&self) -> String {
        #[cfg(all(feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_label_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_label_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_label_TextTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_language_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `language` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/language)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn language(&self) -> String {
        #[cfg(all(feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_language_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_language_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_language_TextTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/id)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_TextTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_in_band_metadata_track_dispatch_type_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `inBandMetadataTrackDispatchType` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/inBandMetadataTrackDispatchType)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn in_band_metadata_track_dispatch_type(&self) -> String {
        #[cfg(all(feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_in_band_metadata_track_dispatch_type_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_in_band_metadata_track_dispatch_type_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_in_band_metadata_track_dispatch_type_TextTrack(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_mode_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <TextTrackMode as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
    #[allow(clippy::all)]
    pub fn mode(&self) -> TextTrackMode {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_mode_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <TextTrackMode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_mode_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <TextTrackMode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_mode_TextTrack(self_)
            };
            <TextTrackMode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackMode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_mode_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrack as WasmDescribe>::describe();
    <TextTrackMode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackMode",))]
    #[allow(bad_style)]
    #[doc = "The `mode` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackMode`*"]
    #[allow(clippy::all)]
    pub fn set_mode(&self, mode: TextTrackMode) {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackMode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_mode_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mode: <TextTrackMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_mode_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mode: <TextTrackMode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(mode);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let mode = <TextTrackMode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mode);
                __widl_f_set_mode_TextTrack(self_, mode)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackCueList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cues_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <Option<TextTrackCueList> as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackCueList",))]
    #[allow(bad_style)]
    #[doc = "The `cues` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/cues)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
    #[allow(clippy::all)]
    pub fn cues(&self) -> Option<TextTrackCueList> {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackCueList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cues_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrackCueList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cues_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrackCueList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cues_TextTrack(self_)
            };
            <Option<TextTrackCueList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackCueList",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_active_cues_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <Option<TextTrackCueList> as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackCueList",))]
    #[allow(bad_style)]
    #[doc = "The `activeCues` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/activeCues)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCueList`*"]
    #[allow(clippy::all)]
    pub fn active_cues(&self) -> Option<TextTrackCueList> {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackCueList",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_active_cues_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrackCueList> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_active_cues_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrackCueList> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_active_cues_TextTrack(self_)
            };
            <Option<TextTrackCueList> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncuechange_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrack as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `oncuechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn oncuechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncuechange_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncuechange_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncuechange_TextTrack(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrack",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncuechange_TextTrack() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrack as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrack {
    #[cfg(all(feature = "TextTrack",))]
    #[allow(bad_style)]
    #[doc = "The `oncuechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)\n\n*This API requires the following crate features to be activated: `TextTrack`*"]
    #[allow(clippy::all)]
    pub fn set_oncuechange(&self, oncuechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TextTrack",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncuechange_TextTrack(
                self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncuechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncuechange_TextTrack(
            self_: <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncuechange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncuechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrack as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncuechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncuechange,
                    );
                __widl_f_set_oncuechange_TextTrack(self_, oncuechange)
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
pub static __WASM_BINDGEN_GENERATED_c58f35d82fd10d0c: [u8; 1148usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}:\x04\0\0\0\0\x0E\0\0\x02\tTextTrack\x1B__widl_instanceof_TextTrack\0\0\0\0\x1A__widl_f_add_cue_TextTrack\0\0\0\x01\tTextTrack\x01\0\0\x01\x02\x05self_\x03cue\x06addCue\0\0\0\x1D__widl_f_remove_cue_TextTrack\x01\0\0\x01\tTextTrack\x01\0\0\x01\x02\x05self_\x03cue\tremoveCue\0\0\0\x17__widl_f_kind_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x04kind\x01\x01\x05self_\x04kind\0\0\0\x18__widl_f_label_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x05label\x01\x01\x05self_\x05label\0\0\0\x1B__widl_f_language_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x08language\x01\x01\x05self_\x08language\0\0\0\x15__widl_f_id_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\07__widl_f_in_band_metadata_track_dispatch_type_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x1FinBandMetadataTrackDispatchType\x01\x01\x05self_\x1FinBandMetadataTrackDispatchType\0\0\0\x17__widl_f_mode_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x04mode\x01\x01\x05self_\x04mode\0\0\0\x1B__widl_f_set_mode_TextTrack\0\0\0\x01\tTextTrack\x01\0\x02\x04mode\x01\x02\x05self_\x04mode\x04mode\0\0\0\x17__widl_f_cues_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x04cues\x01\x01\x05self_\x04cues\0\0\0\x1E__widl_f_active_cues_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\nactiveCues\x01\x01\x05self_\nactiveCues\0\0\0\x1E__widl_f_oncuechange_TextTrack\0\0\0\x01\tTextTrack\x01\0\x01\x0Boncuechange\x01\x01\x05self_\x0Boncuechange\0\0\0\"__widl_f_set_oncuechange_TextTrack\0\0\0\x01\tTextTrack\x01\0\x02\x0Boncuechange\x01\x02\x05self_\x0Boncuechange\x0Boncuechange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
