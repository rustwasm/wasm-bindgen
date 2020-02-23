use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BiquadFilterNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BiquadFilterNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BiquadFilterNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BiquadFilterNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(16u32);
            inform(66u32);
            inform(105u32);
            inform(113u32);
            inform(117u32);
            inform(97u32);
            inform(100u32);
            inform(70u32);
            inform(105u32);
            inform(108u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for BiquadFilterNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for BiquadFilterNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BiquadFilterNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BiquadFilterNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BiquadFilterNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BiquadFilterNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BiquadFilterNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BiquadFilterNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BiquadFilterNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BiquadFilterNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BiquadFilterNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BiquadFilterNode {
        #[inline]
        fn from(obj: JsValue) -> BiquadFilterNode {
            BiquadFilterNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BiquadFilterNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BiquadFilterNode> for BiquadFilterNode {
        #[inline]
        fn as_ref(&self) -> &BiquadFilterNode {
            self
        }
    }
    impl From<BiquadFilterNode> for JsValue {
        #[inline]
        fn from(obj: BiquadFilterNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BiquadFilterNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BiquadFilterNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BiquadFilterNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BiquadFilterNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BiquadFilterNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BiquadFilterNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BiquadFilterNode> for AudioNode {
    #[inline]
    fn from(obj: BiquadFilterNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for BiquadFilterNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<BiquadFilterNode> for EventTarget {
    #[inline]
    fn from(obj: BiquadFilterNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for BiquadFilterNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<BiquadFilterNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: BiquadFilterNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BiquadFilterNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <BiquadFilterNode as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<BiquadFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_BiquadFilterNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_BiquadFilterNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_BiquadFilterNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "BiquadFilterNode",
    feature = "BiquadFilterOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&BiquadFilterOptions as WasmDescribe>::describe();
    <BiquadFilterNode as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "BiquadFilterNode",
        feature = "BiquadFilterOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`, `BiquadFilterOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &BiquadFilterOptions,
    ) -> Result<BiquadFilterNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "BiquadFilterNode",
            feature = "BiquadFilterOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_BiquadFilterNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&BiquadFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_BiquadFilterNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&BiquadFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&BiquadFilterOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_BiquadFilterNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<BiquadFilterNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_get_frequency_response_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <&mut [f32] as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `getFrequencyResponse()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn get_frequency_response(
        &self,
        frequency_hz: &mut [f32],
        mag_response: &mut [f32],
        phase_response: &mut [f32],
    ) {
        #[cfg(all(feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_get_frequency_response_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                frequency_hz: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                mag_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                phase_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_get_frequency_response_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            frequency_hz: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            mag_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            phase_response: <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(frequency_hz);
            drop(mag_response);
            drop(phase_response);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let frequency_hz =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(frequency_hz);
                let mag_response =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(mag_response);
                let phase_response =
                    <&mut [f32] as wasm_bindgen::convert::IntoWasmAbi>::into_abi(phase_response);
                __widl_f_get_frequency_response_BiquadFilterNode(
                    self_,
                    frequency_hz,
                    mag_response,
                    phase_response,
                )
            };
            ()
        }
    }
}
#[cfg(all(feature = "BiquadFilterNode", feature = "BiquadFilterType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_type_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <BiquadFilterType as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "BiquadFilterNode", feature = "BiquadFilterType",))]
    #[allow(bad_style)]
    #[doc = "The `type` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
    #[allow(clippy::all)]
    pub fn type_(&self) -> BiquadFilterType {
        #[cfg(all(feature = "BiquadFilterNode", feature = "BiquadFilterType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_type_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <BiquadFilterType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_type_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <BiquadFilterType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_type_BiquadFilterNode(self_)
            };
            <BiquadFilterType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BiquadFilterNode", feature = "BiquadFilterType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_type_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <BiquadFilterType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "BiquadFilterNode", feature = "BiquadFilterType",))]
    #[allow(bad_style)]
    #[doc = "The `type` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)\n\n*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
    #[allow(clippy::all)]
    pub fn set_type(&self, type_: BiquadFilterType) {
        #[cfg(all(feature = "BiquadFilterNode", feature = "BiquadFilterType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_type_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                type_: <BiquadFilterType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_type_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            type_: <BiquadFilterType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
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
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let type_ =
                    <BiquadFilterType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(type_);
                __widl_f_set_type_BiquadFilterNode(self_, type_)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_frequency_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `frequency` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/frequency)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn frequency(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_frequency_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_frequency_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_frequency_BiquadFilterNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_detune_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `detune` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/detune)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn detune(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_detune_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_detune_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_detune_BiquadFilterNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_q_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `Q` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/Q)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn q(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_q_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_q_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_q_BiquadFilterNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_gain_BiquadFilterNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BiquadFilterNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl BiquadFilterNode {
    #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
    #[allow(bad_style)]
    #[doc = "The `gain` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/gain)\n\n*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
    #[allow(clippy::all)]
    pub fn gain(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "BiquadFilterNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_gain_BiquadFilterNode(
                self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_gain_BiquadFilterNode(
            self_: <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&BiquadFilterNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_gain_BiquadFilterNode(self_)
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
pub static __WASM_BINDGEN_GENERATED_a44eed22ac0c4f67: [u8; 944usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}n\x03\0\0\0\0\n\0\0\x02\x10BiquadFilterNode\"__widl_instanceof_BiquadFilterNode\0\0\0\0\x1D__widl_f_new_BiquadFilterNode\x01\0\0\x01\x10BiquadFilterNode\0\x01\x01\x07context\x03new\0\0\0*__widl_f_new_with_options_BiquadFilterNode\x01\0\0\x01\x10BiquadFilterNode\0\x01\x02\x07context\x07options\x03new\0\0\00__widl_f_get_frequency_response_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\0\x01\x04\x05self_\x0Cfrequency_hz\x0Cmag_response\x0Ephase_response\x14getFrequencyResponse\0\0\0\x1E__widl_f_type_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\x01\x04type\x01\x01\x05self_\x04type\0\0\0\"__widl_f_set_type_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\x02\x04type\x01\x02\x05self_\x05type_\x04type\0\0\0#__widl_f_frequency_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\x01\tfrequency\x01\x01\x05self_\tfrequency\0\0\0 __widl_f_detune_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\x01\x06detune\x01\x01\x05self_\x06detune\0\0\0\x1B__widl_f_q_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\x01\x01Q\x01\x01\x05self_\x01Q\0\0\0\x1E__widl_f_gain_BiquadFilterNode\0\0\0\x01\x10BiquadFilterNode\x01\0\x01\x04gain\x01\x01\x05self_\x04gain\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
