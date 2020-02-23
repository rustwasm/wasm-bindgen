use super::*;
use js_sys::Object;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `PannerNode` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct PannerNode {
    obj: AudioNode,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_PannerNode: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for PannerNode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(10u32);
            inform(80u32);
            inform(97u32);
            inform(110u32);
            inform(110u32);
            inform(101u32);
            inform(114u32);
            inform(78u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for PannerNode {
        type Target = AudioNode;
        #[inline]
        fn deref(&self) -> &AudioNode {
            &self.obj
        }
    }
    impl IntoWasmAbi for PannerNode {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for PannerNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a PannerNode {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for PannerNode {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            PannerNode {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for PannerNode {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a PannerNode {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for PannerNode {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<PannerNode>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(PannerNode {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for PannerNode {
        #[inline]
        fn from(obj: JsValue) -> PannerNode {
            PannerNode { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for PannerNode {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<PannerNode> for PannerNode {
        #[inline]
        fn as_ref(&self) -> &PannerNode {
            self
        }
    }
    impl From<PannerNode> for JsValue {
        #[inline]
        fn from(obj: PannerNode) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for PannerNode {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_PannerNode(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_PannerNode(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_PannerNode(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            PannerNode { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const PannerNode) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<PannerNode> for AudioNode {
    #[inline]
    fn from(obj: PannerNode) -> AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<AudioNode> for PannerNode {
    #[inline]
    fn as_ref(&self) -> &AudioNode {
        use wasm_bindgen::JsCast;
        AudioNode::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PannerNode> for EventTarget {
    #[inline]
    fn from(obj: PannerNode) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for PannerNode {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<PannerNode> for ::js_sys::Object {
    #[inline]
    fn from(obj: PannerNode) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for PannerNode {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BaseAudioContext", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <PannerNode as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "BaseAudioContext", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn new(context: &BaseAudioContext) -> Result<PannerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(feature = "BaseAudioContext", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_PannerNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_PannerNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                __widl_f_new_PannerNode(context)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PannerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(
    feature = "BaseAudioContext",
    feature = "PannerNode",
    feature = "PannerOptions",
))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_new_with_options_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BaseAudioContext as WasmDescribe>::describe();
    <&PannerOptions as WasmDescribe>::describe();
    <PannerNode as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(
        feature = "BaseAudioContext",
        feature = "PannerNode",
        feature = "PannerOptions",
    ))]
    #[allow(bad_style)]
    #[doc = "The `new PannerNode(..)` constructor, creating a new instance of `PannerNode`\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/PannerNode)\n\n*This API requires the following crate features to be activated: `BaseAudioContext`, `PannerNode`, `PannerOptions`*"]
    #[allow(clippy::all)]
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &PannerOptions,
    ) -> Result<PannerNode, ::wasm_bindgen::JsValue> {
        #[cfg(all(
            feature = "BaseAudioContext",
            feature = "PannerNode",
            feature = "PannerOptions",
        ))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_new_with_options_PannerNode(
                context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                options: <&PannerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_new_with_options_PannerNode(
            context: <&BaseAudioContext as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            options: <&PannerOptions as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PannerNode as wasm_bindgen::convert::FromWasmAbi>::Abi {
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
                    <&PannerOptions as wasm_bindgen::convert::IntoWasmAbi>::into_abi(options);
                __widl_f_new_with_options_PannerNode(context, options)
            };
            wasm_bindgen::__rt::take_last_exception()?;
            Ok(<PannerNode as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret))
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_orientation_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `setOrientation()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setOrientation)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_orientation(&self, x: f64, y: f64, z: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_orientation_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_orientation_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_set_orientation_PannerNode(self_, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_position_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `setPosition()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setPosition)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_position(&self, x: f64, y: f64, z: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_position_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_position_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_set_position_PannerNode(self_, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_velocity_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(4u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `setVelocity()` method\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setVelocity)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_velocity(&self, x: f64, y: f64, z: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_velocity_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_velocity_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            x: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            y: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            z: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(x);
            drop(y);
            drop(z);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let x = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(x);
                let y = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(y);
                let z = <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(z);
                __widl_f_set_velocity_PannerNode(self_, x, y, z)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode", feature = "PanningModelType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_panning_model_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <PanningModelType as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode", feature = "PanningModelType",))]
    #[allow(bad_style)]
    #[doc = "The `panningModel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)\n\n*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
    #[allow(clippy::all)]
    pub fn panning_model(&self) -> PanningModelType {
        #[cfg(all(feature = "PannerNode", feature = "PanningModelType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_panning_model_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <PanningModelType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_panning_model_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <PanningModelType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_panning_model_PannerNode(self_)
            };
            <PanningModelType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode", feature = "PanningModelType",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_panning_model_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <PanningModelType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode", feature = "PanningModelType",))]
    #[allow(bad_style)]
    #[doc = "The `panningModel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)\n\n*This API requires the following crate features to be activated: `PannerNode`, `PanningModelType`*"]
    #[allow(clippy::all)]
    pub fn set_panning_model(&self, panning_model: PanningModelType) {
        #[cfg(all(feature = "PannerNode", feature = "PanningModelType",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_panning_model_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                panning_model: <PanningModelType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_panning_model_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            panning_model: <PanningModelType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(panning_model);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let panning_model =
                    <PanningModelType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        panning_model,
                    );
                __widl_f_set_panning_model_PannerNode(self_, panning_model)
            };
            ()
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_x_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `positionX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionX)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn position_x(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_x_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_x_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_x_PannerNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_y_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `positionY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionY)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn position_y(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_y_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_y_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_y_PannerNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_position_z_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `positionZ` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionZ)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn position_z(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_position_z_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_position_z_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_position_z_PannerNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orientation_x_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `orientationX` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationX)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn orientation_x(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orientation_x_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orientation_x_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orientation_x_PannerNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orientation_y_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `orientationY` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationY)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn orientation_y(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orientation_y_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orientation_y_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orientation_y_PannerNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_orientation_z_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <AudioParam as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `orientationZ` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationZ)\n\n*This API requires the following crate features to be activated: `AudioParam`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn orientation_z(&self) -> AudioParam {
        #[cfg(all(feature = "AudioParam", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_orientation_z_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_orientation_z_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <AudioParam as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_orientation_z_PannerNode(self_)
            };
            <AudioParam as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DistanceModelType", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_distance_model_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <DistanceModelType as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "DistanceModelType", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `distanceModel` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)\n\n*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn distance_model(&self) -> DistanceModelType {
        #[cfg(all(feature = "DistanceModelType", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_distance_model_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <DistanceModelType as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_distance_model_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <DistanceModelType as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_distance_model_PannerNode(self_)
            };
            <DistanceModelType as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "DistanceModelType", feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_distance_model_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <DistanceModelType as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "DistanceModelType", feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `distanceModel` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)\n\n*This API requires the following crate features to be activated: `DistanceModelType`, `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_distance_model(&self, distance_model: DistanceModelType) {
        #[cfg(all(feature = "DistanceModelType", feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_distance_model_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                distance_model: <DistanceModelType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_distance_model_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            distance_model: <DistanceModelType as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(distance_model);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let distance_model =
                    <DistanceModelType as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        distance_model,
                    );
                __widl_f_set_distance_model_PannerNode(self_, distance_model)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ref_distance_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `refDistance` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn ref_distance(&self) -> f64 {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ref_distance_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ref_distance_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ref_distance_PannerNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ref_distance_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `refDistance` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_ref_distance(&self, ref_distance: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ref_distance_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ref_distance: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ref_distance_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ref_distance: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(ref_distance);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ref_distance =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(ref_distance);
                __widl_f_set_ref_distance_PannerNode(self_, ref_distance)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_max_distance_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `maxDistance` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn max_distance(&self) -> f64 {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_max_distance_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_max_distance_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_max_distance_PannerNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_max_distance_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `maxDistance` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_max_distance(&self, max_distance: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_max_distance_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                max_distance: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_max_distance_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            max_distance: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(max_distance);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let max_distance =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(max_distance);
                __widl_f_set_max_distance_PannerNode(self_, max_distance)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_rolloff_factor_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `rolloffFactor` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn rolloff_factor(&self) -> f64 {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_rolloff_factor_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_rolloff_factor_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_rolloff_factor_PannerNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_rolloff_factor_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `rolloffFactor` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_rolloff_factor(&self, rolloff_factor: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_rolloff_factor_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                rolloff_factor: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_rolloff_factor_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            rolloff_factor: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(rolloff_factor);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let rolloff_factor =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(rolloff_factor);
                __widl_f_set_rolloff_factor_PannerNode(self_, rolloff_factor)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cone_inner_angle_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `coneInnerAngle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn cone_inner_angle(&self) -> f64 {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cone_inner_angle_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cone_inner_angle_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cone_inner_angle_PannerNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cone_inner_angle_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `coneInnerAngle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_cone_inner_angle(&self, cone_inner_angle: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cone_inner_angle_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cone_inner_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cone_inner_angle_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cone_inner_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cone_inner_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cone_inner_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cone_inner_angle);
                __widl_f_set_cone_inner_angle_PannerNode(self_, cone_inner_angle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cone_outer_angle_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `coneOuterAngle` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn cone_outer_angle(&self) -> f64 {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cone_outer_angle_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cone_outer_angle_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cone_outer_angle_PannerNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cone_outer_angle_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `coneOuterAngle` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_cone_outer_angle(&self, cone_outer_angle: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cone_outer_angle_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cone_outer_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cone_outer_angle_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cone_outer_angle: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cone_outer_angle);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cone_outer_angle =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cone_outer_angle);
                __widl_f_set_cone_outer_angle_PannerNode(self_, cone_outer_angle)
            };
            ()
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_cone_outer_gain_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `coneOuterGain` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn cone_outer_gain(&self) -> f64 {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_cone_outer_gain_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_cone_outer_gain_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_cone_outer_gain_PannerNode(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "PannerNode",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_cone_outer_gain_PannerNode() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&PannerNode as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl PannerNode {
    #[cfg(all(feature = "PannerNode",))]
    #[allow(bad_style)]
    #[doc = "The `coneOuterGain` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)\n\n*This API requires the following crate features to be activated: `PannerNode`*"]
    #[allow(clippy::all)]
    pub fn set_cone_outer_gain(&self, cone_outer_gain: f64) {
        #[cfg(all(feature = "PannerNode",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_cone_outer_gain_PannerNode(
                self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                cone_outer_gain: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_cone_outer_gain_PannerNode(
            self_: <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            cone_outer_gain: <f64 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(cone_outer_gain);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&PannerNode as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let cone_outer_gain =
                    <f64 as wasm_bindgen::convert::IntoWasmAbi>::into_abi(cone_outer_gain);
                __widl_f_set_cone_outer_gain_PannerNode(self_, cone_outer_gain)
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
pub static __WASM_BINDGEN_GENERATED_f3aa005d35cce494: [u8; 2649usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\x17\n\0\0\0\0\x1C\0\0\x02\nPannerNode\x1C__widl_instanceof_PannerNode\0\0\0\0\x17__widl_f_new_PannerNode\x01\0\0\x01\nPannerNode\0\x01\x01\x07context\x03new\0\0\0$__widl_f_new_with_options_PannerNode\x01\0\0\x01\nPannerNode\0\x01\x02\x07context\x07options\x03new\0\0\0#__widl_f_set_orientation_PannerNode\0\0\0\x01\nPannerNode\x01\0\0\x01\x04\x05self_\x01x\x01y\x01z\x0EsetOrientation\0\0\0 __widl_f_set_position_PannerNode\0\0\0\x01\nPannerNode\x01\0\0\x01\x04\x05self_\x01x\x01y\x01z\x0BsetPosition\0\0\0 __widl_f_set_velocity_PannerNode\0\0\0\x01\nPannerNode\x01\0\0\x01\x04\x05self_\x01x\x01y\x01z\x0BsetVelocity\0\0\0!__widl_f_panning_model_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0CpanningModel\x01\x01\x05self_\x0CpanningModel\0\0\0%__widl_f_set_panning_model_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\x0CpanningModel\x01\x02\x05self_\rpanning_model\x0CpanningModel\0\0\0\x1E__widl_f_position_x_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\tpositionX\x01\x01\x05self_\tpositionX\0\0\0\x1E__widl_f_position_y_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\tpositionY\x01\x01\x05self_\tpositionY\0\0\0\x1E__widl_f_position_z_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\tpositionZ\x01\x01\x05self_\tpositionZ\0\0\0!__widl_f_orientation_x_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0CorientationX\x01\x01\x05self_\x0CorientationX\0\0\0!__widl_f_orientation_y_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0CorientationY\x01\x01\x05self_\x0CorientationY\0\0\0!__widl_f_orientation_z_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0CorientationZ\x01\x01\x05self_\x0CorientationZ\0\0\0\"__widl_f_distance_model_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\rdistanceModel\x01\x01\x05self_\rdistanceModel\0\0\0&__widl_f_set_distance_model_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\rdistanceModel\x01\x02\x05self_\x0Edistance_model\rdistanceModel\0\0\0 __widl_f_ref_distance_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0BrefDistance\x01\x01\x05self_\x0BrefDistance\0\0\0$__widl_f_set_ref_distance_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\x0BrefDistance\x01\x02\x05self_\x0Cref_distance\x0BrefDistance\0\0\0 __widl_f_max_distance_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0BmaxDistance\x01\x01\x05self_\x0BmaxDistance\0\0\0$__widl_f_set_max_distance_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\x0BmaxDistance\x01\x02\x05self_\x0Cmax_distance\x0BmaxDistance\0\0\0\"__widl_f_rolloff_factor_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\rrolloffFactor\x01\x01\x05self_\rrolloffFactor\0\0\0&__widl_f_set_rolloff_factor_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\rrolloffFactor\x01\x02\x05self_\x0Erolloff_factor\rrolloffFactor\0\0\0$__widl_f_cone_inner_angle_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0EconeInnerAngle\x01\x01\x05self_\x0EconeInnerAngle\0\0\0(__widl_f_set_cone_inner_angle_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\x0EconeInnerAngle\x01\x02\x05self_\x10cone_inner_angle\x0EconeInnerAngle\0\0\0$__widl_f_cone_outer_angle_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\x0EconeOuterAngle\x01\x01\x05self_\x0EconeOuterAngle\0\0\0(__widl_f_set_cone_outer_angle_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\x0EconeOuterAngle\x01\x02\x05self_\x10cone_outer_angle\x0EconeOuterAngle\0\0\0#__widl_f_cone_outer_gain_PannerNode\0\0\0\x01\nPannerNode\x01\0\x01\rconeOuterGain\x01\x01\x05self_\rconeOuterGain\0\0\0'__widl_f_set_cone_outer_gain_PannerNode\0\0\0\x01\nPannerNode\x01\0\x02\rconeOuterGain\x01\x02\x05self_\x0Fcone_outer_gain\rconeOuterGain\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
