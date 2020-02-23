use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `OscillatorNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct OscillatorNode {
    obj: AudioScheduledSourceNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_OscillatorNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for OscillatorNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(79u32);
            inform(115u32);
            inform(99u32);
            inform(105u32);
            inform(108u32);
            inform(108u32);
            inform(97u32);
            inform(116u32);
            inform(111u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for OscillatorNode {
        type Target = AudioScheduledSourceNode;
        #[inline]
        fn deref(&self) -> &AudioScheduledSourceNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for OscillatorNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for OscillatorNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a OscillatorNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for OscillatorNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            OscillatorNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for OscillatorNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a OscillatorNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for OscillatorNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<OscillatorNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(OscillatorNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for OscillatorNode {
        #[inline]
        fn from(obj: JsValue) -> OscillatorNode {
            OscillatorNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for OscillatorNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<OscillatorNode> for OscillatorNode {
        #[inline]
        fn as_ref(&self) -> &OscillatorNode {
            self
        }
    }
    impl From<OscillatorNode> for JsValue {
        #[inline]
        fn from(obj: OscillatorNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for OscillatorNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_OscillatorNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_OscillatorNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_OscillatorNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            OscillatorNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const OscillatorNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<OscillatorNode> for AudioScheduledSourceNode {
    #[inline]
    fn from(obj: OscillatorNode) -> AudioScheduledSourceNode {
        use wasm_bindgen::JsCast;
        AudioScheduledSourceNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioScheduledSourceNode> for OscillatorNode {
    #[inline]
    fn as_ref(&self) -> &AudioScheduledSourceNode {
        use wasm_bindgen::JsCast;
        AudioScheduledSourceNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OscillatorNode> for AudioNode {
    #[inline]
    fn from(obj: OscillatorNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for OscillatorNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OscillatorNode> for EventTarget {
    #[inline]
    fn from(obj: OscillatorNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for OscillatorNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<OscillatorNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: OscillatorNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for OscillatorNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <OscillatorNode as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<OscillatorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_OscillatorNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_OscillatorNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_OscillatorNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "OscillatorNode",
    feature = "OscillatorOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&OscillatorOptions as WasmDescribe>::describe();
    <OscillatorNode as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "OscillatorNode",
        feature = "OscillatorOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`, `OscillatorOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &OscillatorOptions,
    ) -> Result<OscillatorNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "OscillatorNode",
            feature = "OscillatorOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_OscillatorNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&OscillatorOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_OscillatorNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&OscillatorOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&OscillatorOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_OscillatorNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<OscillatorNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "OscillatorNode", feature = "PeriodicWave",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_periodic_wave_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <&PeriodicWave as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode", feature = "PeriodicWave",))]
    #[allow(bad_style)]
    #[doc = "The `setPeriodicWave()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/setPeriodicWave)\n\n*This API requires the following crate features to be activated: `OscillatorNode`, `PeriodicWave`*"]
    #[allow(clippy::all)]
    pub fn set_periodic_wave(&self, periodic_wave: &PeriodicWave) {
        #[cfg(all(feature = "OscillatorNode", feature = "PeriodicWave",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_periodic_wave_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                periodic_wave: <&PeriodicWave as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_periodic_wave_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            periodic_wave: <&PeriodicWave as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(periodic_wave);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let periodic_wave =
                    <&PeriodicWave as wasm_bindgen::convert::IntoWasmAbi>::into_abi(periodic_wave);
                __widl_f_set_periodic_wave_OscillatorNode(self_, periodic_wave)
            };
            ()
        }
    }
}
#[cfg(all(feature = "OscillatorNode", feature = "OscillatorType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <OscillatorType as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode", feature = "OscillatorType",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)\n\n*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> OscillatorType {
        #[cfg(all(feature = "OscillatorNode", feature = "OscillatorType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <OscillatorType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <OscillatorType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_OscillatorNode(self_)
            };
            <OscillatorType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OscillatorNode", feature = "OscillatorType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <OscillatorType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode", feature = "OscillatorType",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)\n\n*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: OscillatorType) {
        #[cfg(all(feature = "OscillatorNode", feature = "OscillatorType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <OscillatorType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <OscillatorType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(type_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ = <OscillatorType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_OscillatorNode(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frequency_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "AudioParam", feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `frequency` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/frequency)\n\n*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn frequency(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frequency_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frequency_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frequency_OscillatorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detune_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "AudioParam", feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `detune` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/detune)\n\n*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn detune(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detune_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detune_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detune_OscillatorNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn start(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_start_OscillatorNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_start_with_when_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `start()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn start_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_start_with_when_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_start_with_when_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(when);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_start_with_when_OscillatorNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn stop(&self) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_stop_OscillatorNode(self_)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_stop_with_when_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `stop()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn stop_with_when(&self, when: f64) -> Result<(), ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_stop_with_when_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_stop_with_when_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            when: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(when);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let when = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(when);
                __widl_f_stop_with_when_OscillatorNode(self_, when)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(())
        }
    }
}
#[cfg(all(feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onended_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn onended(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onended_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onended_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onended_OscillatorNode(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "OscillatorNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onended_OscillatorNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&OscillatorNode as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl OscillatorNode {
    #[cfg(all(feature = "OscillatorNode",))]
    #[allow(bad_style)]
    #[doc = "The `onended` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)\n\n*This API requires the following crate features to be activated: `OscillatorNode`*"]
    #[allow(clippy::all)]
    pub fn set_onended(&self, onended: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "OscillatorNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onended_OscillatorNode(
                self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onended_OscillatorNode(
            self_: <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onended: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onended);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&OscillatorNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onended =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onended,
                    );
                __widl_f_set_onended_OscillatorNode(self_, onended)
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
pub static __WASM_BINDGEN_GENERATED_fe24a495ce4f682f: [u8; 1208usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}v\x04\0\0\0\0\x0E\0\0\x02\x0EOscillatorNode __widl_instanceof_OscillatorNode\0\0\0\0\x1B__widl_f_new_OscillatorNode\x01\0\0\x01\x0EOscillatorNode\0\x01\x01\x07context\x03new\0\0\0(__widl_f_new_with_options_OscillatorNode\x01\0\0\x01\x0EOscillatorNode\0\x01\x02\x07context\x07options\x03new\0\0\0)__widl_f_set_periodic_wave_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\0\x01\x02\x05self_\rperiodic_wave\x0FsetPeriodicWave\0\0\0\x1C__widl_f_type_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0 __widl_f_set_type_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0!__widl_f_frequency_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\x01\tfrequency\x01\x01\x05self_\tfrequency\0\0\0\x1E__widl_f_detune_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\x01\x06detune\x01\x01\x05self_\x06detune\0\0\0\x1D__widl_f_start_OscillatorNode\x01\0\0\x01\x0EOscillatorNode\x01\0\0\x01\x01\x05self_\x05start\0\0\0'__widl_f_start_with_when_OscillatorNode\x01\0\0\x01\x0EOscillatorNode\x01\0\0\x01\x02\x05self_\x04when\x05start\0\0\0\x1C__widl_f_stop_OscillatorNode\x01\0\0\x01\x0EOscillatorNode\x01\0\0\x01\x01\x05self_\x04stop\0\0\0&__widl_f_stop_with_when_OscillatorNode\x01\0\0\x01\x0EOscillatorNode\x01\0\0\x01\x02\x05self_\x04when\x04stop\0\0\0\x1F__widl_f_onended_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\x01\x07onended\x01\x01\x05self_\x07onended\0\0\0#__widl_f_set_onended_OscillatorNode\0\0\0\x01\x0EOscillatorNode\x01\0\x02\x07onended\x01\x02\x05self_\x07onended\x07onended\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
