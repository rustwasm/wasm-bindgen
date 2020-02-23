use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `TextTrackCue` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct TextTrackCue {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_TextTrackCue: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for TextTrackCue {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(12u32);
            inform(84u32);
            inform(101u32);
            inform(120u32);
            inform(116u32);
            inform(84u32);
            inform(114u32);
            inform(97u32);
            inform(99u32);
            inform(107u32);
            inform(67u32);
            inform(117u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for TextTrackCue {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for TextTrackCue {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for TextTrackCue {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a TextTrackCue {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for TextTrackCue {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            TextTrackCue {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for TextTrackCue {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a TextTrackCue {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for TextTrackCue {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<TextTrackCue>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(TextTrackCue {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for TextTrackCue {
        #[inline]
        fn from(obj: JsValue) -> TextTrackCue {
            TextTrackCue { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for TextTrackCue {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<TextTrackCue> for TextTrackCue {
        #[inline]
        fn as_ref(&self) -> &TextTrackCue {
            self
        }
    }
    impl From<TextTrackCue> for JsValue {
        #[inline]
        fn from(obj: TextTrackCue) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for TextTrackCue {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_TextTrackCue(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_TextTrackCue(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_TextTrackCue(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            TextTrackCue { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const TextTrackCue) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<TextTrackCue> for EventTarget {
    #[inline]
    fn from(obj: TextTrackCue) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for TextTrackCue {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<TextTrackCue> for ::js_sys::Object {
    #[inline]
    fn from(obj: TextTrackCue) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for TextTrackCue {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "TextTrack", feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_track_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <Option<TextTrack> as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrack", feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `track` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/track)\n\n*This API requires the following crate features to be activated: `TextTrack`, `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn track(&self) -> Option<TextTrack> {
        #[cfg(all(feature = "TextTrack", feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_track_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_track_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_track_TextTrackCue(self_)
            };
            <Option<TextTrack> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_TextTrackCue(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_id_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `id` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/id)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn set_id(&self, id: &str) {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_id_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_id_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(id);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_set_id_TextTrackCue(self_, id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_time_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `startTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn start_time(&self) -> f64 {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_time_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_time_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_time_TextTrackCue(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_start_time_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `startTime` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/startTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn set_start_time(&self, start_time: f64) {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_start_time_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_start_time_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(start_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                __widl_f_set_start_time_TextTrackCue(self_, start_time)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_end_time_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `endTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn end_time(&self) -> f64 {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_end_time_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_end_time_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_end_time_TextTrackCue(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_end_time_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `endTime` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/endTime)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn set_end_time(&self, end_time: f64) {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_end_time_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_end_time_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            end_time: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(end_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let end_time = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(end_time);
                __widl_f_set_end_time_TextTrackCue(self_, end_time)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pause_on_exit_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `pauseOnExit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn pause_on_exit(&self) -> bool {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pause_on_exit_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pause_on_exit_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pause_on_exit_TextTrackCue(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pause_on_exit_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `pauseOnExit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/pauseOnExit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn set_pause_on_exit(&self, pause_on_exit: bool) {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pause_on_exit_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pause_on_exit: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pause_on_exit_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pause_on_exit: <bool as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pause_on_exit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pause_on_exit =
                    <bool as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pause_on_exit);
                __widl_f_set_pause_on_exit_TextTrackCue(self_, pause_on_exit)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onenter_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `onenter` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn onenter(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onenter_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onenter_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onenter_TextTrackCue(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onenter_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `onenter` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onenter)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn set_onenter(&self, onenter: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onenter_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onenter_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onenter: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onenter);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onenter =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onenter,
                    );
                __widl_f_set_onenter_TextTrackCue(self_, onenter)
            };
            ()
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onexit_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `onexit` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn onexit(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onexit_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onexit_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onexit_TextTrackCue(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "TextTrackCue",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onexit_TextTrackCue() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&TextTrackCue as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl TextTrackCue {
    #[cfg(all(feature = "TextTrackCue",))]
    #[allow(bad_style)]
    #[doc = "The `onexit` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackCue/onexit)\n\n*This API requires the following crate features to be activated: `TextTrackCue`*"]
    #[allow(clippy::all)]
    pub fn set_onexit(&self, onexit: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "TextTrackCue",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onexit_TextTrackCue(
                self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onexit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onexit_TextTrackCue(
            self_: <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onexit: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onexit);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&TextTrackCue as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onexit =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onexit,
                    );
                __widl_f_set_onexit_TextTrackCue(self_, onexit)
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
pub static __WASM_BINDGEN_GENERATED_d905d91a1b983161: [u8; 1232usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x8E\x04\0\0\0\0\x0E\0\0\x02\x0CTextTrackCue\x1E__widl_instanceof_TextTrackCue\0\0\0\0\x1B__widl_f_track_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\x05track\x01\x01\x05self_\x05track\0\0\0\x18__widl_f_id_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x1C__widl_f_set_id_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x02\x02id\x01\x02\x05self_\x02id\x02id\0\0\0 __widl_f_start_time_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\tstartTime\x01\x01\x05self_\tstartTime\0\0\0$__widl_f_set_start_time_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x02\tstartTime\x01\x02\x05self_\nstart_time\tstartTime\0\0\0\x1E__widl_f_end_time_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\x07endTime\x01\x01\x05self_\x07endTime\0\0\0\"__widl_f_set_end_time_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x02\x07endTime\x01\x02\x05self_\x08end_time\x07endTime\0\0\0#__widl_f_pause_on_exit_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\x0BpauseOnExit\x01\x01\x05self_\x0BpauseOnExit\0\0\0'__widl_f_set_pause_on_exit_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x02\x0BpauseOnExit\x01\x02\x05self_\rpause_on_exit\x0BpauseOnExit\0\0\0\x1D__widl_f_onenter_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\x07onenter\x01\x01\x05self_\x07onenter\0\0\0!__widl_f_set_onenter_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x02\x07onenter\x01\x02\x05self_\x07onenter\x07onenter\0\0\0\x1C__widl_f_onexit_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x01\x06onexit\x01\x01\x05self_\x06onexit\0\0\0 __widl_f_set_onexit_TextTrackCue\0\0\0\x01\x0CTextTrackCue\x01\0\x02\x06onexit\x01\x02\x05self_\x06onexit\x06onexit\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
