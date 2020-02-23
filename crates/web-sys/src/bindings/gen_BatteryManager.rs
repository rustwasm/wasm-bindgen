use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `BatteryManager` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct BatteryManager {
    obj: EventTarget,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_BatteryManager: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for BatteryManager {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(14u32);
            inform(66u32);
            inform(97u32);
            inform(116u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(121u32);
            inform(77u32);
            inform(97u32);
            inform(110u32);
            inform(97u32);
            inform(103u32);
            inform(101u32);
            inform(114u32);
        }
    }
    impl core::ops::Deref for BatteryManager {
        type Target = EventTarget;
        #[inline]
        fn deref(&self) -> &EventTarget {
            &self.obj
        }
    }
    impl IntoWasmAbi for BatteryManager {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for BatteryManager {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a BatteryManager {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for BatteryManager {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            BatteryManager {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for BatteryManager {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a BatteryManager {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for BatteryManager {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<BatteryManager>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(BatteryManager {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for BatteryManager {
        #[inline]
        fn from(obj: JsValue) -> BatteryManager {
            BatteryManager { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for BatteryManager {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<BatteryManager> for BatteryManager {
        #[inline]
        fn as_ref(&self) -> &BatteryManager {
            self
        }
    }
    impl From<BatteryManager> for JsValue {
        #[inline]
        fn from(obj: BatteryManager) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for BatteryManager {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_BatteryManager(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_BatteryManager(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_BatteryManager(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            BatteryManager { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const BatteryManager) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<BatteryManager> for EventTarget {
    #[inline]
    fn from(obj: BatteryManager) -> EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<EventTarget> for BatteryManager {
    #[inline]
    fn as_ref(&self) -> &EventTarget {
        use wasm_bindgen::JsCast;
        EventTarget::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<BatteryManager> for ::js_sys::Object {
    #[inline]
    fn from(obj: BatteryManager) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for BatteryManager {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_charging_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <bool as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `charging` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/charging)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn charging(&self) -> bool {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_charging_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_charging_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <bool as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_charging_BatteryManager(self_)
            };
            <bool as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_charging_time_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `chargingTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/chargingTime)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn charging_time(&self) -> f64 {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_charging_time_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_charging_time_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_charging_time_BatteryManager(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_discharging_time_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `dischargingTime` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/dischargingTime)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn discharging_time(&self) -> f64 {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_discharging_time_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_discharging_time_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_discharging_time_BatteryManager(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_level_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <f64 as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `level` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/level)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn level(&self) -> f64 {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_level_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_level_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <f64 as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_level_BatteryManager(self_)
            };
            <f64 as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchargingchange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `onchargingchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn onchargingchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchargingchange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchargingchange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchargingchange_BatteryManager(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchargingchange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `onchargingchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn set_onchargingchange(&self, onchargingchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchargingchange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchargingchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchargingchange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchargingchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onchargingchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchargingchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchargingchange,
                    );
                __widl_f_set_onchargingchange_BatteryManager(self_, onchargingchange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onchargingtimechange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `onchargingtimechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn onchargingtimechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onchargingtimechange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onchargingtimechange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onchargingtimechange_BatteryManager(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onchargingtimechange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `onchargingtimechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn set_onchargingtimechange(&self, onchargingtimechange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onchargingtimechange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onchargingtimechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onchargingtimechange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onchargingtimechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(onchargingtimechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onchargingtimechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onchargingtimechange,
                    );
                __widl_f_set_onchargingtimechange_BatteryManager(self_, onchargingtimechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_ondischargingtimechange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `ondischargingtimechange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn ondischargingtimechange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_ondischargingtimechange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_ondischargingtimechange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_ondischargingtimechange_BatteryManager(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_ondischargingtimechange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `ondischargingtimechange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn set_ondischargingtimechange(
        &self,
        ondischargingtimechange: Option<&::js_sys::Function>,
    ) {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_ondischargingtimechange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                ondischargingtimechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_ondischargingtimechange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ondischargingtimechange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
        ) -> () {
            drop(self_);
            drop(ondischargingtimechange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let ondischargingtimechange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        ondischargingtimechange,
                    );
                __widl_f_set_ondischargingtimechange_BatteryManager(self_, ondischargingtimechange)
            };
            ()
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_onlevelchange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<::js_sys::Function> as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `onlevelchange` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn onlevelchange(&self) -> Option<::js_sys::Function> {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_onlevelchange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_onlevelchange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_onlevelchange_BatteryManager(self_)
            };
            <Option<::js_sys::Function> as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "BatteryManager",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_onlevelchange_BatteryManager() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&BatteryManager as WasmDescribe>::describe();
    <Option<&::js_sys::Function> as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl BatteryManager {
    #[cfg(all(feature = "BatteryManager",))]
    #[allow(bad_style)]
    #[doc = "The `onlevelchange` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)\n\n*This API requires the following crate features to be activated: `BatteryManager`*"]
    #[allow(clippy::all)]
    pub fn set_onlevelchange(&self, onlevelchange: Option<&::js_sys::Function>) {
        #[cfg(all(feature = "BatteryManager",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_onlevelchange_BatteryManager(
                self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                onlevelchange : < Option < & :: js_sys :: Function > as wasm_bindgen :: convert :: IntoWasmAbi > :: Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_onlevelchange_BatteryManager(
            self_: <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            onlevelchange: <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(onlevelchange);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ = <&BatteryManager as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let onlevelchange =
                    <Option<&::js_sys::Function> as wasm_bindgen::convert::IntoWasmAbi>::into_abi(
                        onlevelchange,
                    );
                __widl_f_set_onlevelchange_BatteryManager(self_, onlevelchange)
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
pub static __WASM_BINDGEN_GENERATED_a3b428d9f31cd978: [u8; 1523usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB1\x05\0\0\0\0\r\0\0\x02\x0EBatteryManager __widl_instanceof_BatteryManager\0\0\0\0 __widl_f_charging_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x08charging\x01\x01\x05self_\x08charging\0\0\0%__widl_f_charging_time_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x0CchargingTime\x01\x01\x05self_\x0CchargingTime\0\0\0(__widl_f_discharging_time_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x0FdischargingTime\x01\x01\x05self_\x0FdischargingTime\0\0\0\x1D__widl_f_level_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x05level\x01\x01\x05self_\x05level\0\0\0(__widl_f_onchargingchange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x10onchargingchange\x01\x01\x05self_\x10onchargingchange\0\0\0,__widl_f_set_onchargingchange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x02\x10onchargingchange\x01\x02\x05self_\x10onchargingchange\x10onchargingchange\0\0\0,__widl_f_onchargingtimechange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x14onchargingtimechange\x01\x01\x05self_\x14onchargingtimechange\0\0\00__widl_f_set_onchargingtimechange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x02\x14onchargingtimechange\x01\x02\x05self_\x14onchargingtimechange\x14onchargingtimechange\0\0\0/__widl_f_ondischargingtimechange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\x17ondischargingtimechange\x01\x01\x05self_\x17ondischargingtimechange\0\0\03__widl_f_set_ondischargingtimechange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x02\x17ondischargingtimechange\x01\x02\x05self_\x17ondischargingtimechange\x17ondischargingtimechange\0\0\0%__widl_f_onlevelchange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x01\ronlevelchange\x01\x01\x05self_\ronlevelchange\0\0\0)__widl_f_set_onlevelchange_BatteryManager\0\0\0\x01\x0EBatteryManager\x01\0\x02\ronlevelchange\x01\x02\x05self_\ronlevelchange\ronlevelchange\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
