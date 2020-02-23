use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `Animation` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation)\n\n*This API requires the following crate features to be activated: `Animation`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct Animation {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_Animation: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for Animation {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(9u32);
            inform(65u32);
            inform(110u32);
            inform(105u32);
            inform(109u32);
            inform(97u32);
            inform(116u32);
            inform(105u32);
            inform(111u32);
            inform(110u32);
        }
    }
    impl core::ops::Deref for Animation {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for Animation {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for Animation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a Animation {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for Animation {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            Animation {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for Animation {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a Animation {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for Animation {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<Animation>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(Animation {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for Animation {
        #[inline]
        fn from(obj: JsValue) -> Animation {
            Animation { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for Animation {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<Animation> for Animation {
        #[inline]
        fn as_ref(&self) -> &Animation {
            self
        }
    }
    impl From<Animation> for JsValue {
        #[inline]
        fn from(obj: Animation) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for Animation {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_Animation(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_Animation(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_Animation(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            Animation { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const Animation) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<Animation> for EventTarget {
    #[inline]
    fn from(obj: Animation) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for Animation {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<Animation> for ::js_sys::Object {
    #[inline]
    fn from(obj: Animation) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for Animation {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(0u32);
    <Animation as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn new() -> Result<Animation, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_Animation() -> <Animation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_Animation() -> <Animation as wasm_bindgen::convert::FromWasmAbi>::Abi
        {
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = { __widl_f_new_Animation() };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Animation as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_effect_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <Option<&AnimationEffect> as WasmDescribe>::describe();
    <Animation as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
    #[allow(bad_style)]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
    #[allow(clippy::all)]
    pub fn new_with_effect(
        effect: Option<&AnimationEffect>,
    ) -> Result<Animation, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_effect_Animation(
                effect: <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Animation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_effect_Animation(
            effect: <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Animation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(effect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let effect =
                    <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        effect,
                    );
                __widl_f_new_with_effect_Animation(effect)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Animation as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "Animation",
    feature = "AnimationEffect",
    feature = "AnimationTimeline",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_effect_and_timeline_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&AnimationEffect> as WasmDescribe>::describe();
    <Option<&AnimationTimeline> as WasmDescribe>::describe();
    <Animation as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(
        feature = "Animation",
        feature = "AnimationEffect",
        feature = "AnimationTimeline",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`, `AnimationTimeline`*"]
    #[allow(clippy::all)]
    pub fn new_with_effect_and_timeline(
        effect: Option<&AnimationEffect>,
        timeline: Option<&AnimationTimeline>,
    ) -> Result<Animation, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Animation",
            feature = "AnimationEffect",
            feature = "AnimationTimeline",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_effect_and_timeline_Animation(
                effect: <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeline: <Option<&AnimationTimeline> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Animation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_effect_and_timeline_Animation(
            effect: <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeline: <Option<&AnimationTimeline> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Animation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(effect);
            drop(timeline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let effect =
                    <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        effect,
                    );
                let timeline =
                    <Option<&AnimationTimeline> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        timeline,
                    );
                __widl_f_new_with_effect_and_timeline_Animation(effect, timeline)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<Animation as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cancel_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `cancel()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/cancel)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn cancel(&self) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cancel_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cancel_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cancel_Animation(self_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_finish_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `finish()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finish)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn finish(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_finish_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_finish_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_finish_Animation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pause_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `pause()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pause)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn pause(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pause_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pause_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pause_Animation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_play_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `play()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/play)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn play(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_play_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_play_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_play_Animation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_reverse_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `reverse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/reverse)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn reverse(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_reverse_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_reverse_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_reverse_Animation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_update_playback_rate_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `updatePlaybackRate()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/updatePlaybackRate)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn update_playback_rate(&self, playback_rate: f64) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_update_playback_rate_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_update_playback_rate_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(playback_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let playback_rate =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(playback_rate);
                __widl_f_update_playback_rate_Animation(self_, playback_rate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_id_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `id` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn id(&self) -> String {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_id_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_id_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_id_Animation(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_id_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `id` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn set_id(&self, id: &str) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_id_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                id: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_id_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let id = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(id);
                __widl_f_set_id_Animation(self_, id)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_effect_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <Option<AnimationEffect> as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
    #[allow(bad_style)]
    #[doc = "The `effect` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
    #[allow(clippy::all)]
    pub fn effect(&self) -> Option<AnimationEffect> {
        #[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_effect_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<AnimationEffect> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_effect_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<AnimationEffect> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_effect_Animation(self_)
            };
            <Option<AnimationEffect> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_effect_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <Option<&AnimationEffect> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
    #[allow(bad_style)]
    #[doc = "The `effect` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
    #[allow(clippy::all)]
    pub fn set_effect(&self, effect: Option<&AnimationEffect>) {
        #[cfg(all(feature = "Animation", feature = "AnimationEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_effect_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                effect: <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_effect_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            effect: <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(effect);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let effect =
                    <Option<&AnimationEffect> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        effect,
                    );
                __widl_f_set_effect_Animation(self_, effect)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation", feature = "AnimationTimeline",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_timeline_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <Option<AnimationTimeline> as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation", feature = "AnimationTimeline",))]
    #[allow(bad_style)]
    #[doc = "The `timeline` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*"]
    #[allow(clippy::all)]
    pub fn timeline(&self) -> Option<AnimationTimeline> {
        #[cfg(all(feature = "Animation", feature = "AnimationTimeline",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_timeline_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<AnimationTimeline> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_timeline_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<AnimationTimeline> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_timeline_Animation(self_)
            };
            <Option<AnimationTimeline> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation", feature = "AnimationTimeline",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_timeline_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <Option<&AnimationTimeline> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation", feature = "AnimationTimeline",))]
    #[allow(bad_style)]
    #[doc = "The `timeline` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*"]
    #[allow(clippy::all)]
    pub fn set_timeline(&self, timeline: Option<&AnimationTimeline>) {
        #[cfg(all(feature = "Animation", feature = "AnimationTimeline",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_timeline_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                timeline: <Option<&AnimationTimeline> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_timeline_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            timeline: <Option<&AnimationTimeline> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(timeline);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let timeline =
                    <Option<&AnimationTimeline> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        timeline,
                    );
                __widl_f_set_timeline_Animation(self_, timeline)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_time_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `startTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn start_time(&self) -> Option<f64> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_time_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_time_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_time_Animation(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_start_time_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `startTime` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn set_start_time(&self, start_time: Option<f64>) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_start_time_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                start_time: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_start_time_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            start_time: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let start_time =
                    <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(start_time);
                __widl_f_set_start_time_Animation(self_, start_time)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_current_time_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn current_time(&self) -> Option<f64> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_current_time_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_current_time_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_current_time_Animation(self_)
            };
            <Option<f64> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_current_time_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <Option<f64> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `currentTime` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn set_current_time(&self, current_time: Option<f64>) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_current_time_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                current_time: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_current_time_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            current_time: <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(current_time);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let current_time =
                    <Option<f64> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(current_time);
                __widl_f_set_current_time_Animation(self_, current_time)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_playback_rate_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `playbackRate` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn playback_rate(&self) -> f64 {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_playback_rate_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_playback_rate_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_playback_rate_Animation(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_playback_rate_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `playbackRate` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn set_playback_rate(&self, playback_rate: f64) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_playback_rate_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_playback_rate_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            playback_rate: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(playback_rate);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let playback_rate =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(playback_rate);
                __widl_f_set_playback_rate_Animation(self_, playback_rate)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation", feature = "AnimationPlayState",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_play_state_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <AnimationPlayState as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation", feature = "AnimationPlayState",))]
    #[allow(bad_style)]
    #[doc = "The `playState` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playState)\n\n*This API requires the following crate features to be activated: `Animation`, `AnimationPlayState`*"]
    #[allow(clippy::all)]
    pub fn play_state(&self) -> AnimationPlayState {
        #[cfg(all(feature = "Animation", feature = "AnimationPlayState",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_play_state_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AnimationPlayState as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_play_state_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AnimationPlayState as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_play_state_Animation(self_)
            };
            <AnimationPlayState as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pending_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `pending` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pending)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn pending(&self) -> bool {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pending_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pending_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pending_Animation(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ready_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `ready` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/ready)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn ready(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ready_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ready_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ready_Animation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_finished_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <::js_sys::Promise as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `finished` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finished)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn finished(&self) -> Result<::js_sys::Promise, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_finished_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_finished_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_finished_Animation(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Promise as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onfinish_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `onfinish` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn onfinish(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onfinish_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onfinish_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onfinish_Animation(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onfinish_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `onfinish` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn set_onfinish(&self, onfinish: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onfinish_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onfinish: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onfinish_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onfinish: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onfinish);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onfinish =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onfinish,
                    );
                __widl_f_set_onfinish_Animation(self_, onfinish)
            };
            ()
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_oncancel_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&Animation as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `oncancel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn oncancel(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_oncancel_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_oncancel_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_oncancel_Animation(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "Animation",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_oncancel_Animation() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&Animation as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl Animation {
    #[cfg(all(feature = "Animation",))]
    #[allow(bad_style)]
    #[doc = "The `oncancel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)\n\n*This API requires the following crate features to be activated: `Animation`*"]
    #[allow(clippy::all)]
    pub fn set_oncancel(&self, oncancel: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "Animation",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_oncancel_Animation(
                self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                oncancel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_oncancel_Animation(
            self_: <&Animation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            oncancel: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(oncancel);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&Animation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let oncancel =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        oncancel,
                    );
                __widl_f_set_oncancel_Animation(self_, oncancel)
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
pub static __WASM_BINDGEN_GENERATED_a3421b4847084926: [u8; 2335usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xDD\x08\0\0\0\0\x1E\0\0\x02\tAnimation\x1B__widl_instanceof_Animation\0\0\0\0\x16__widl_f_new_Animation\x01\0\0\x01\tAnimation\0\x01\0\x03new\0\0\0\"__widl_f_new_with_effect_Animation\x01\0\0\x01\tAnimation\0\x01\x01\x06effect\x03new\0\0\0/__widl_f_new_with_effect_and_timeline_Animation\x01\0\0\x01\tAnimation\0\x01\x02\x06effect\x08timeline\x03new\0\0\0\x19__widl_f_cancel_Animation\0\0\0\x01\tAnimation\x01\0\0\x01\x01\x05self_\x06cancel\0\0\0\x19__widl_f_finish_Animation\x01\0\0\x01\tAnimation\x01\0\0\x01\x01\x05self_\x06finish\0\0\0\x18__widl_f_pause_Animation\x01\0\0\x01\tAnimation\x01\0\0\x01\x01\x05self_\x05pause\0\0\0\x17__widl_f_play_Animation\x01\0\0\x01\tAnimation\x01\0\0\x01\x01\x05self_\x04play\0\0\0\x1A__widl_f_reverse_Animation\x01\0\0\x01\tAnimation\x01\0\0\x01\x01\x05self_\x07reverse\0\0\0'__widl_f_update_playback_rate_Animation\0\0\0\x01\tAnimation\x01\0\0\x01\x02\x05self_\rplayback_rate\x12updatePlaybackRate\0\0\0\x15__widl_f_id_Animation\0\0\0\x01\tAnimation\x01\0\x01\x02id\x01\x01\x05self_\x02id\0\0\0\x19__widl_f_set_id_Animation\0\0\0\x01\tAnimation\x01\0\x02\x02id\x01\x02\x05self_\x02id\x02id\0\0\0\x19__widl_f_effect_Animation\0\0\0\x01\tAnimation\x01\0\x01\x06effect\x01\x01\x05self_\x06effect\0\0\0\x1D__widl_f_set_effect_Animation\0\0\0\x01\tAnimation\x01\0\x02\x06effect\x01\x02\x05self_\x06effect\x06effect\0\0\0\x1B__widl_f_timeline_Animation\0\0\0\x01\tAnimation\x01\0\x01\x08timeline\x01\x01\x05self_\x08timeline\0\0\0\x1F__widl_f_set_timeline_Animation\0\0\0\x01\tAnimation\x01\0\x02\x08timeline\x01\x02\x05self_\x08timeline\x08timeline\0\0\0\x1D__widl_f_start_time_Animation\0\0\0\x01\tAnimation\x01\0\x01\tstartTime\x01\x01\x05self_\tstartTime\0\0\0!__widl_f_set_start_time_Animation\0\0\0\x01\tAnimation\x01\0\x02\tstartTime\x01\x02\x05self_\nstart_time\tstartTime\0\0\0\x1F__widl_f_current_time_Animation\0\0\0\x01\tAnimation\x01\0\x01\x0BcurrentTime\x01\x01\x05self_\x0BcurrentTime\0\0\0#__widl_f_set_current_time_Animation\0\0\0\x01\tAnimation\x01\0\x02\x0BcurrentTime\x01\x02\x05self_\x0Ccurrent_time\x0BcurrentTime\0\0\0 __widl_f_playback_rate_Animation\0\0\0\x01\tAnimation\x01\0\x01\x0CplaybackRate\x01\x01\x05self_\x0CplaybackRate\0\0\0$__widl_f_set_playback_rate_Animation\0\0\0\x01\tAnimation\x01\0\x02\x0CplaybackRate\x01\x02\x05self_\rplayback_rate\x0CplaybackRate\0\0\0\x1D__widl_f_play_state_Animation\0\0\0\x01\tAnimation\x01\0\x01\tplayState\x01\x01\x05self_\tplayState\0\0\0\x1A__widl_f_pending_Animation\0\0\0\x01\tAnimation\x01\0\x01\x07pending\x01\x01\x05self_\x07pending\0\0\0\x18__widl_f_ready_Animation\x01\0\0\x01\tAnimation\x01\0\x01\x05ready\x01\x01\x05self_\x05ready\0\0\0\x1B__widl_f_finished_Animation\x01\0\0\x01\tAnimation\x01\0\x01\x08finished\x01\x01\x05self_\x08finished\0\0\0\x1B__widl_f_onfinish_Animation\0\0\0\x01\tAnimation\x01\0\x01\x08onfinish\x01\x01\x05self_\x08onfinish\0\0\0\x1F__widl_f_set_onfinish_Animation\0\0\0\x01\tAnimation\x01\0\x02\x08onfinish\x01\x02\x05self_\x08onfinish\x08onfinish\0\0\0\x1B__widl_f_oncancel_Animation\0\0\0\x01\tAnimation\x01\0\x01\x08oncancel\x01\x01\x05self_\x08oncancel\0\0\0\x1F__widl_f_set_oncancel_Animation\0\0\0\x01\tAnimation\x01\0\x02\x08oncancel\x01\x02\x05self_\x08oncancel\x08oncancel\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
