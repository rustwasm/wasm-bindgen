use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `GainNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode)\n\n*This API requires the following crate features to be activated: `GainNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct GainNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_GainNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for GainNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(8u32);
            inform(71u32);
            inform(97u32);
            inform(105u32);
            inform(110u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for GainNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for GainNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for GainNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a GainNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for GainNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            GainNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for GainNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a GainNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for GainNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<GainNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(GainNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for GainNode {
        #[inline]
        fn from(obj: JsValue) -> GainNode {
            GainNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for GainNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<GainNode> for GainNode {
        #[inline]
        fn as_ref(&self) -> &GainNode {
            self
        }
    }
    impl From<GainNode> for JsValue {
        #[inline]
        fn from(obj: GainNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for GainNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_GainNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_GainNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_GainNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            GainNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const GainNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<GainNode> for AudioNode {
    #[inline]
    fn from(obj: GainNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for GainNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GainNode> for EventTarget {
    #[inline]
    fn from(obj: GainNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for GainNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<GainNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: GainNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for GainNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "GainNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_GainNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <GainNode as WasmDescribe>::describe();
}
impl GainNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "GainNode",))]
    #[allow(bad_style)]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<GainNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "GainNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_GainNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_GainNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(context);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let context =
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                __widl_f_new_GainNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GainNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "GainNode",
    feature = "GainOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_GainNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&GainOptions as WasmDescribe>::describe();
    <GainNode as WasmDescribe>::describe();
}
impl GainNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "GainNode",
        feature = "GainOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`, `GainOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &GainOptions,
    ) -> Result<GainNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "GainNode",
            feature = "GainOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_GainNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&GainOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_GainNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&GainOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <GainNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::into_abi(context);
                let options =
                    <&GainOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_GainNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<GainNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                _ret,
            ))
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "GainNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_gain_GainNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&GainNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl GainNode {
    #[cfg(all(feature = "AudioParam", feature = "GainNode",))]
    #[allow(bad_style)]
    #[doc = "The `gain` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/gain)\n\n*This API requires the following crate features to be activated: `AudioParam`, `GainNode`*"]
    #[allow(clippy::all)]
    pub fn gain(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "GainNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_gain_GainNode(
                self_: <&GainNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_gain_GainNode(
            self_: <&GainNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&GainNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_gain_GainNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
#[link_section = "__wasm_bindgen_unstable"]
#[doc(hidden)]
#[allow(clippy::all)]
pub static __WASM_BINDGEN_GENERATED_e1eacf2f0fcba3db: [u8; 332usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\n\x01\0\0\0\0\x04\0\0\x02\x08GainNode\x1A__widl_instanceof_GainNode\0\0\0\0\x15__widl_f_new_GainNode\x01\0\0\x01\x08GainNode\0\x01\x01\x07context\x03new\0\0\0\"__widl_f_new_with_options_GainNode\x01\0\0\x01\x08GainNode\0\x01\x02\x07context\x07options\x03new\0\0\0\x16__widl_f_gain_GainNode\0\0\0\x01\x08GainNode\x01\0\x01\x04gain\x01\x01\x05self_\x04gain\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
