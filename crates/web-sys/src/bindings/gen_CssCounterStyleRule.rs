use super::*;
#[allow(bad_style)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "The `CSSCounterStyleRule` object\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
#[repr(transparent)]
#[allow(clippy::all)]
pub struct CssCounterStyleRule {
    obj: CssRule,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CssCounterStyleRule: () = {
    use wasm_bindgen::__rt::core;
    use wasm_bindgen::convert::RefFromWasmAbi;
    use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
    use wasm_bindgen::convert::{OptionFromWasmAbi, OptionIntoWasmAbi};
    use wasm_bindgen::describe::WasmDescribe;
    use wasm_bindgen::{JsCast, JsValue};
    impl WasmDescribe for CssCounterStyleRule {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(NAMED_ANYREF);
            inform(19u32);
            inform(67u32);
            inform(83u32);
            inform(83u32);
            inform(67u32);
            inform(111u32);
            inform(117u32);
            inform(110u32);
            inform(116u32);
            inform(101u32);
            inform(114u32);
            inform(83u32);
            inform(116u32);
            inform(121u32);
            inform(108u32);
            inform(101u32);
            inform(82u32);
            inform(117u32);
            inform(108u32);
            inform(101u32);
        }
    }
    impl core::ops::Deref for CssCounterStyleRule {
        type Target = CssRule;
        #[inline]
        fn deref(&self) -> &CssRule {
            &self.obj
        }
    }
    impl IntoWasmAbi for CssCounterStyleRule {
        type Abi = <JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            self.obj.into_abi()
        }
    }
    impl OptionIntoWasmAbi for CssCounterStyleRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl<'a> OptionIntoWasmAbi for &'a CssCounterStyleRule {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    impl FromWasmAbi for CssCounterStyleRule {
        type Abi = <JsValue as FromWasmAbi>::Abi;
        #[inline]
        unsafe fn from_abi(js: Self::Abi) -> Self {
            CssCounterStyleRule {
                obj: JsValue::from_abi(js).into(),
            }
        }
    }
    impl OptionFromWasmAbi for CssCounterStyleRule {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    impl<'a> IntoWasmAbi for &'a CssCounterStyleRule {
        type Abi = <&'a JsValue as IntoWasmAbi>::Abi;
        #[inline]
        fn into_abi(self) -> Self::Abi {
            (&self.obj).into_abi()
        }
    }
    impl RefFromWasmAbi for CssCounterStyleRule {
        type Abi = <JsValue as RefFromWasmAbi>::Abi;
        type Anchor = core::mem::ManuallyDrop<CssCounterStyleRule>;
        #[inline]
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            let tmp = <JsValue as RefFromWasmAbi>::ref_from_abi(js);
            core::mem::ManuallyDrop::new(CssCounterStyleRule {
                obj: core::mem::ManuallyDrop::into_inner(tmp).into(),
            })
        }
    }
    impl From<JsValue> for CssCounterStyleRule {
        #[inline]
        fn from(obj: JsValue) -> CssCounterStyleRule {
            CssCounterStyleRule { obj: obj.into() }
        }
    }
    impl AsRef<JsValue> for CssCounterStyleRule {
        #[inline]
        fn as_ref(&self) -> &JsValue {
            self.obj.as_ref()
        }
    }
    impl AsRef<CssCounterStyleRule> for CssCounterStyleRule {
        #[inline]
        fn as_ref(&self) -> &CssCounterStyleRule {
            self
        }
    }
    impl From<CssCounterStyleRule> for JsValue {
        #[inline]
        fn from(obj: CssCounterStyleRule) -> JsValue {
            obj.obj.into()
        }
    }
    impl JsCast for CssCounterStyleRule {
        fn instanceof(val: &JsValue) -> bool {
            #[link(wasm_import_module = "__wbindgen_placeholder__")]
            #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
            extern "C" {
                fn __widl_instanceof_CSSCounterStyleRule(val: u32) -> u32;
            }
            #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
            unsafe fn __widl_instanceof_CSSCounterStyleRule(_: u32) -> u32 {
                panic!("cannot check instanceof on non-wasm targets");
            }
            unsafe {
                let idx = val.into_abi();
                __widl_instanceof_CSSCounterStyleRule(idx) != 0
            }
        }
        #[inline]
        fn unchecked_from_js(val: JsValue) -> Self {
            CssCounterStyleRule { obj: val.into() }
        }
        #[inline]
        fn unchecked_from_js_ref(val: &JsValue) -> &Self {
            unsafe { &*(val as *const JsValue as *const CssCounterStyleRule) }
        }
    }
    ()
};
#[allow(clippy::all)]
impl From<CssCounterStyleRule> for CssRule {
    #[inline]
    fn from(obj: CssCounterStyleRule) -> CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<CssRule> for CssCounterStyleRule {
    #[inline]
    fn as_ref(&self) -> &CssRule {
        use wasm_bindgen::JsCast;
        CssRule::unchecked_from_js_ref(self.as_ref())
    }
}
#[allow(clippy::all)]
impl From<CssCounterStyleRule> for ::js_sys::Object {
    #[inline]
    fn from(obj: CssCounterStyleRule) -> ::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js(obj.into())
    }
}
#[allow(clippy::all)]
impl AsRef<::js_sys::Object> for CssCounterStyleRule {
    #[inline]
    fn as_ref(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsCast;
        ::js_sys::Object::unchecked_from_js_ref(self.as_ref())
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_name_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `name` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn name(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_name_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_name_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_name_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_name_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `name` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/name)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_name(&self, name: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_name_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_name_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            name: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(name);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let name = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(name);
                __widl_f_set_name_CSSCounterStyleRule(self_, name)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_system_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `system` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn system(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_system_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_system_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_system_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_system_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `system` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/system)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_system(&self, system: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_system_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                system: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_system_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            system: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(system);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let system = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(system);
                __widl_f_set_system_CSSCounterStyleRule(self_, system)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_symbols_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `symbols` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn symbols(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_symbols_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_symbols_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_symbols_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_symbols_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `symbols` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/symbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_symbols(&self, symbols: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_symbols_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                symbols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_symbols_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            symbols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(symbols);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let symbols = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(symbols);
                __widl_f_set_symbols_CSSCounterStyleRule(self_, symbols)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_additive_symbols_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `additiveSymbols` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn additive_symbols(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_additive_symbols_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_additive_symbols_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_additive_symbols_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_additive_symbols_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `additiveSymbols` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/additiveSymbols)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_additive_symbols(&self, additive_symbols: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_additive_symbols_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                additive_symbols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_additive_symbols_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            additive_symbols: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(additive_symbols);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let additive_symbols =
                    <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(additive_symbols);
                __widl_f_set_additive_symbols_CSSCounterStyleRule(self_, additive_symbols)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_negative_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `negative` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn negative(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_negative_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_negative_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_negative_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_negative_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `negative` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/negative)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_negative(&self, negative: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_negative_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                negative: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_negative_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            negative: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(negative);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let negative = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(negative);
                __widl_f_set_negative_CSSCounterStyleRule(self_, negative)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_prefix_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `prefix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn prefix(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_prefix_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_prefix_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_prefix_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_prefix_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `prefix` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/prefix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_prefix(&self, prefix: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_prefix_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                prefix: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_prefix_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            prefix: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(prefix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let prefix = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(prefix);
                __widl_f_set_prefix_CSSCounterStyleRule(self_, prefix)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_suffix_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `suffix` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn suffix(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_suffix_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_suffix_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_suffix_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_suffix_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `suffix` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/suffix)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_suffix(&self, suffix: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_suffix_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                suffix: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_suffix_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            suffix: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(suffix);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let suffix = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(suffix);
                __widl_f_set_suffix_CSSCounterStyleRule(self_, suffix)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_range_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `range` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn range(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_range_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_range_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_range_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_range_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `range` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/range)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_range(&self, range: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_range_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                range: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_range_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            range: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(range);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let range = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(range);
                __widl_f_set_range_CSSCounterStyleRule(self_, range)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_pad_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `pad` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn pad(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_pad_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_pad_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_pad_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_pad_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `pad` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/pad)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_pad(&self, pad: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_pad_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                pad: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_pad_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            pad: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(pad);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let pad = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(pad);
                __widl_f_set_pad_CSSCounterStyleRule(self_, pad)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_speak_as_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `speakAs` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn speak_as(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_speak_as_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_speak_as_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_speak_as_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_speak_as_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `speakAs` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/speakAs)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_speak_as(&self, speak_as: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_speak_as_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                speak_as: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_speak_as_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            speak_as: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(speak_as);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let speak_as = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(speak_as);
                __widl_f_set_speak_as_CSSCounterStyleRule(self_, speak_as)
            };
            ()
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_fallback_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(1u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <String as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `fallback` getter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn fallback(&self) -> String {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_fallback_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi;
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_fallback_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> <String as wasm_bindgen::convert::FromWasmAbi>::Abi {
            drop(self_);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                __widl_f_fallback_CSSCounterStyleRule(self_)
            };
            <String as wasm_bindgen::convert::FromWasmAbi>::from_abi(_ret)
        }
    }
}
#[cfg(all(feature = "CssCounterStyleRule",))]
#[no_mangle]
#[allow(non_snake_case)]
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
#[allow(clippy::all)]
pub extern "C" fn __wbindgen_describe___widl_f_set_fallback_CSSCounterStyleRule() {
    use wasm_bindgen::describe::*;
    wasm_bindgen::__rt::link_mem_intrinsics();
    inform(FUNCTION);
    inform(0);
    inform(2u32);
    <&CssCounterStyleRule as WasmDescribe>::describe();
    <&str as WasmDescribe>::describe();
    <() as WasmDescribe>::describe();
}
impl CssCounterStyleRule {
    #[cfg(all(feature = "CssCounterStyleRule",))]
    #[allow(bad_style)]
    #[doc = "The `fallback` setter\n\n[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSCounterStyleRule/fallback)\n\n*This API requires the following crate features to be activated: `CssCounterStyleRule`*"]
    #[allow(clippy::all)]
    pub fn set_fallback(&self, fallback: &str) {
        #[cfg(all(feature = "CssCounterStyleRule",))]
        #[link(wasm_import_module = "__wbindgen_placeholder__")]
        #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
        extern "C" {
            fn __widl_f_set_fallback_CSSCounterStyleRule(
                self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
                fallback: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            ) -> ();
        }
        #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
        unsafe fn __widl_f_set_fallback_CSSCounterStyleRule(
            self_: <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            fallback: <&str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
        ) -> () {
            drop(self_);
            drop(fallback);
            panic!(
                "cannot call wasm-bindgen imported functions on \
                            non-wasm targets"
            );
        }
        unsafe {
            let _ret = {
                let self_ =
                    <&CssCounterStyleRule as wasm_bindgen::convert::IntoWasmAbi>::into_abi(self);
                let fallback = <&str as wasm_bindgen::convert::IntoWasmAbi>::into_abi(fallback);
                __widl_f_set_fallback_CSSCounterStyleRule(self_, fallback)
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
pub static __WASM_BINDGEN_GENERATED_263b23661d95223f: [u8; 2293usize] = {
    static _INCLUDED_FILES: &[&str] = &[];
    * b":\0\0\0{\"schema_version\":\"0.2.58\",\"version\":\"0.2.58 (0f3c53b5a)\"}\xB3\x08\0\0\0\0\x17\0\0\x02\x13CSSCounterStyleRule%__widl_instanceof_CSSCounterStyleRule\0\0\0\0!__widl_f_name_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x04name\x01\x01\x05self_\x04name\0\0\0%__widl_f_set_name_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x04name\x01\x02\x05self_\x04name\x04name\0\0\0#__widl_f_system_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x06system\x01\x01\x05self_\x06system\0\0\0'__widl_f_set_system_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x06system\x01\x02\x05self_\x06system\x06system\0\0\0$__widl_f_symbols_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x07symbols\x01\x01\x05self_\x07symbols\0\0\0(__widl_f_set_symbols_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x07symbols\x01\x02\x05self_\x07symbols\x07symbols\0\0\0-__widl_f_additive_symbols_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x0FadditiveSymbols\x01\x01\x05self_\x0FadditiveSymbols\0\0\01__widl_f_set_additive_symbols_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x0FadditiveSymbols\x01\x02\x05self_\x10additive_symbols\x0FadditiveSymbols\0\0\0%__widl_f_negative_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x08negative\x01\x01\x05self_\x08negative\0\0\0)__widl_f_set_negative_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x08negative\x01\x02\x05self_\x08negative\x08negative\0\0\0#__widl_f_prefix_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x06prefix\x01\x01\x05self_\x06prefix\0\0\0'__widl_f_set_prefix_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x06prefix\x01\x02\x05self_\x06prefix\x06prefix\0\0\0#__widl_f_suffix_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x06suffix\x01\x01\x05self_\x06suffix\0\0\0'__widl_f_set_suffix_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x06suffix\x01\x02\x05self_\x06suffix\x06suffix\0\0\0\"__widl_f_range_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x05range\x01\x01\x05self_\x05range\0\0\0&__widl_f_set_range_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x05range\x01\x02\x05self_\x05range\x05range\0\0\0 __widl_f_pad_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x03pad\x01\x01\x05self_\x03pad\0\0\0$__widl_f_set_pad_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x03pad\x01\x02\x05self_\x03pad\x03pad\0\0\0%__widl_f_speak_as_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x07speakAs\x01\x01\x05self_\x07speakAs\0\0\0)__widl_f_set_speak_as_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x07speakAs\x01\x02\x05self_\x08speak_as\x07speakAs\0\0\0%__widl_f_fallback_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x01\x08fallback\x01\x01\x05self_\x08fallback\0\0\0)__widl_f_set_fallback_CSSCounterStyleRule\0\0\0\x01\x13CSSCounterStyleRule\x01\0\x02\x08fallback\x01\x02\x05self_\x08fallback\x08fallback\0\0\0\0\x1Ebuild-web-sys-3a70f0062dec68bf\0"
};
