use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `KeyframeEffect` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct KeyframeEffect {
    obj: AnimationEffect,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_KeyframeEffect: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for KeyframeEffect {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(75u32);
            inform(101u32);
            inform(121u32);
            inform(102u32);
            inform(114u32);
            inform(97u32);
            inform(109u32);
            inform(101u32);
            inform(69u32);
            inform(102u32);
            inform(102u32);
            inform(101u32);
            inform(99u32);
            inform(116u32);
        }
    }
    impl core::ops::Deref for KeyframeEffect {
        type Target = AnimationEffect;
        #[inline]
        fn deref(&self) -> &AnimationEffect {
            &self.obj
        }
    }
    impl IntoWasmAbi for KeyframeEffect {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for KeyframeEffect {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a KeyframeEffect {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for KeyframeEffect {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            KeyframeEffect {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for KeyframeEffect {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a KeyframeEffect {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for KeyframeEffect {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<KeyframeEffect>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(KeyframeEffect {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for KeyframeEffect {
        #[inline]
        fn from(obj: JsValue) -> KeyframeEffect {
            KeyframeEffect { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for KeyframeEffect {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<KeyframeEffect> for KeyframeEffect {
        #[inline]
        fn as_ref(&self) -> &KeyframeEffect {
            self
        }
    }
    impl From<KeyframeEffect> for JsValue {
        #[inline]
        fn from(obj: KeyframeEffect) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for KeyframeEffect {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_KeyframeEffect(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_KeyframeEffect(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_KeyframeEffect(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            KeyframeEffect { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const KeyframeEffect) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<KeyframeEffect> for AnimationEffect {
    #[inline]
    fn from(obj: KeyframeEffect) -> AnimationEffect {
        use wasm_bindgen::JsCast;
        AnimationEffect::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AnimationEffect> for KeyframeEffect {
    #[inline]
    fn as_ref(&self) -> &AnimationEffect {
        use wasm_bindgen::JsCast;
        AnimationEffect::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<KeyframeEffect> for ::js_sys::Object {
    #[inline]
    fn from(obj: KeyframeEffect) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for KeyframeEffect {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "Element", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_element_and_keyframes_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&Element> as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "Element", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_element_and_keyframes(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_element_and_keyframes_KeyframeEffect(
                target: <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_element_and_keyframes_KeyframeEffect(
            target: <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(target);
            drop(keyframes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let target =
                    <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                __widl_f_new_with_opt_element_and_keyframes_KeyframeEffect(target, keyframes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_css_pseudo_element_and_keyframes_KeyframeEffect(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <Option<&CssPseudoElement> as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_css_pseudo_element_and_keyframes(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_css_pseudo_element_and_keyframes_KeyframeEffect(
                target: <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_css_pseudo_element_and_keyframes_KeyframeEffect(
            target: <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(target);
            drop(keyframes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let target =
                    <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        target,
                    );
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                __widl_f_new_with_opt_css_pseudo_element_and_keyframes_KeyframeEffect(
                    target, keyframes,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "Element", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_element_and_keyframes_and_f64_KeyframeEffect(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <Option<&Element> as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "Element", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_element_and_keyframes_and_f64(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "Element", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_element_and_keyframes_and_f64_KeyframeEffect(
                target: <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_element_and_keyframes_and_f64_KeyframeEffect(
            target: <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(target);
            drop(keyframes);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let target =
                    <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                let options = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_opt_element_and_keyframes_and_f64_KeyframeEffect(
                    target, keyframes, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_f64_KeyframeEffect(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <Option<&CssPseudoElement> as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_f64(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: f64,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "CssPseudoElement", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_f64_KeyframeEffect(
                target: <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_f64_KeyframeEffect(
            target: <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(target);
            drop(keyframes);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let target =
                    <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        target,
                    );
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                let options = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_f64_KeyframeEffect(
                    target, keyframes, options,
                )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "Element",
    feature = "KeyframeEffect",
    feature = "KeyframeEffectOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <Option<&Element> as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <&KeyframeEffectOptions as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(
        feature = "Element",
        feature = "KeyframeEffect",
        feature = "KeyframeEffectOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `Element`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_element_and_keyframes_and_keyframe_effect_options(
        target: Option<&Element>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "Element",
            feature = "KeyframeEffect",
            feature = "KeyframeEffectOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect(
                target: <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&KeyframeEffectOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect(
            target: <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&KeyframeEffectOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(target);
            drop(keyframes);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let target =
                    <Option<&Element> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(target);
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                let options =
                    <&KeyframeEffectOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_opt_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect ( target , keyframes , options )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "CssPseudoElement",
    feature = "KeyframeEffect",
    feature = "KeyframeEffectOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect(
) {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(3u32);
    <Option<&CssPseudoElement> as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <&KeyframeEffectOptions as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(
        feature = "CssPseudoElement",
        feature = "KeyframeEffect",
        feature = "KeyframeEffectOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `CssPseudoElement`, `KeyframeEffect`, `KeyframeEffectOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options(
        target: Option<&CssPseudoElement>,
        keyframes: Option<&::js_sys::Object>,
        options: &KeyframeEffectOptions,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "CssPseudoElement",
            feature = "KeyframeEffect",
            feature = "KeyframeEffectOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect(
                target: <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&KeyframeEffectOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect(
            target: <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&KeyframeEffectOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(target);
            drop(keyframes);
            drop(options);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let target =
                    <Option<&CssPseudoElement> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        target,
                    );
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                let options =
                    <&KeyframeEffectOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        options,
                    );
                __widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect ( target , keyframes , options )
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_source_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <KeyframeEffect as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `new KeyframeEffect(..)` constructor, creating a new instance of `KeyframeEffect`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/KeyframeEffect)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn new_with_source(
        source: &KeyframeEffect,
    ) -> Result<KeyframeEffect, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_source_KeyframeEffect(
                source: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_source_KeyframeEffect(
            source: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(source);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let source =
                    <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(source);
                __widl_f_new_with_source_KeyframeEffect(source)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<KeyframeEffect as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_keyframes_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <::js_sys::Array as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `getKeyframes()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/getKeyframes)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn get_keyframes(&self) -> Result<::js_sys::Array, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_keyframes_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_keyframes_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_get_keyframes_KeyframeEffect(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<::js_sys::Array as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_keyframes_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `setKeyframes()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/setKeyframes)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn set_keyframes(
        &self,
        keyframes: Option<&::js_sys::Object>,
    ) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_keyframes_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_keyframes_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            keyframes: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(keyframes);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let keyframes =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        keyframes,
                    );
                __widl_f_set_keyframes_KeyframeEffect(self_, keyframes)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_target_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <Option<::js_sys::Object> as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `target` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn target(&self) -> Option<::js_sys::Object> {
        #[cfg(all(feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_target_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_target_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_target_KeyframeEffect(self_)
            };
            <Option<::js_sys::Object> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_target_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <Option<&::js_sys::Object> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `target` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/target)\n\n*This API requires the following crate features to be activated: `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn set_target(&self, target: Option<&::js_sys::Object>) {
        #[cfg(all(feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_target_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                target: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_target_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            target: <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(target);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let target =
                    <Option<&::js_sys::Object> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        target,
                    );
                __widl_f_set_target_KeyframeEffect(self_, target)
            };
            ()
        }
    }
}
#[cfg(all(feature = "IterationCompositeOperation", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_iteration_composite_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <IterationCompositeOperation as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "IterationCompositeOperation", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `iterationComposite` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)\n\n*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn iteration_composite(&self) -> IterationCompositeOperation {
        #[cfg(all(feature = "IterationCompositeOperation", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_iteration_composite_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <IterationCompositeOperation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_iteration_composite_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <IterationCompositeOperation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_iteration_composite_KeyframeEffect(self_)
            };
            <IterationCompositeOperation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "IterationCompositeOperation", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_iteration_composite_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <IterationCompositeOperation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "IterationCompositeOperation", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `iterationComposite` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/iterationComposite)\n\n*This API requires the following crate features to be activated: `IterationCompositeOperation`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn set_iteration_composite(&self, iteration_composite: IterationCompositeOperation) {
        #[cfg(all(feature = "IterationCompositeOperation", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_iteration_composite_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                iteration_composite : < IterationCompositeOperation as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_iteration_composite_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            iteration_composite : < IterationCompositeOperation as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(iteration_composite);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let iteration_composite =
                    <IterationCompositeOperation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        iteration_composite,
                    );
                __widl_f_set_iteration_composite_KeyframeEffect(self_, iteration_composite)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CompositeOperation", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_composite_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <CompositeOperation as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "CompositeOperation", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `composite` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)\n\n*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn composite(&self) -> CompositeOperation {
        #[cfg(all(feature = "CompositeOperation", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_composite_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <CompositeOperation as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_composite_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <CompositeOperation as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_composite_KeyframeEffect(self_)
            };
            <CompositeOperation as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CompositeOperation", feature = "KeyframeEffect",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_composite_KeyframeEffect() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&KeyframeEffect as WasmDescribe>::describe();
    <CompositeOperation as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl KeyframeEffect {
    #[cfg(all(feature = "CompositeOperation", feature = "KeyframeEffect",))]
    #[allow(bad_style)]
    #[doc = "The `composite` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyframeEffect/composite)\n\n*This API requires the following crate features to be activated: `CompositeOperation`, `KeyframeEffect`*"]
    #[allow(clippy::all)]
    pub fn set_composite(&self, composite: CompositeOperation) {
        #[cfg(all(feature = "CompositeOperation", feature = "KeyframeEffect",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_composite_KeyframeEffect(
                self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                composite: <CompositeOperation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_composite_KeyframeEffect(
            self_: <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            composite: <CompositeOperation as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(composite);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&KeyframeEffect as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let composite =
                    <CompositeOperation as wasm_bindgen::convert::IntoWasmAbi>::into_abi(composite);
                __widl_f_set_composite_KeyframeEffect(self_, composite)
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
pub static __WASM_BINDGEN_GENERATED_4261efe394bc0e31: [u8; 1787usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB9\x06\0\0\0\0\x10\0\0\x02\x0EKeyframeEffect __widl_instanceof_KeyframeEffect\0\0\0\0:__widl_f_new_with_opt_element_and_keyframes_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x02\x06target\tkeyframes\x03new\0\0\0E__widl_f_new_with_opt_css_pseudo_element_and_keyframes_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x02\x06target\tkeyframes\x03new\0\0\0B__widl_f_new_with_opt_element_and_keyframes_and_f64_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x03\x06target\tkeyframes\x07options\x03new\0\0\0M__widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_f64_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x03\x06target\tkeyframes\x07options\x03new\0\0\0V__widl_f_new_with_opt_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x03\x06target\tkeyframes\x07options\x03new\0\0\0a__widl_f_new_with_opt_css_pseudo_element_and_keyframes_and_keyframe_effect_options_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x03\x06target\tkeyframes\x07options\x03new\0\0\0'__widl_f_new_with_source_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\0\x01\x01\x06source\x03new\0\0\0%__widl_f_get_keyframes_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\x01\0\0\x01\x01\x05self_\x0CgetKeyframes\0\0\0%__widl_f_set_keyframes_KeyframeEffect\x01\0\0\x01\x0EKeyframeEffect\x01\0\0\x01\x02\x05self_\tkeyframes\x0CsetKeyframes\0\0\0\x1E__widl_f_target_KeyframeEffect\0\0\0\x01\x0EKeyframeEffect\x01\0\x01\x06target\x01\x01\x05self_\x06target\0\0\0\"__widl_f_set_target_KeyframeEffect\0\0\0\x01\x0EKeyframeEffect\x01\0\x02\x06target\x01\x02\x05self_\x06target\x06target\0\0\0+__widl_f_iteration_composite_KeyframeEffect\0\0\0\x01\x0EKeyframeEffect\x01\0\x01\x12iterationComposite\x01\x01\x05self_\x12iterationComposite\0\0\0/__widl_f_set_iteration_composite_KeyframeEffect\0\0\0\x01\x0EKeyframeEffect\x01\0\x02\x12iterationComposite\x01\x02\x05self_\x13iteration_composite\x12iterationComposite\0\0\0!__widl_f_composite_KeyframeEffect\0\0\0\x01\x0EKeyframeEffect\x01\0\x01\tcomposite\x01\x01\x05self_\tcomposite\0\0\0%__widl_f_set_composite_KeyframeEffect\0\0\0\x01\x0EKeyframeEffect\x01\0\x02\tcomposite\x01\x02\x05self_\tcomposite\tcomposite\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
