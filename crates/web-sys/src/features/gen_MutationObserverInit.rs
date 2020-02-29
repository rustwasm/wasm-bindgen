use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = MutationObserverInit ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `MutationObserverInit` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*
    pub type MutationObserverInit;

}

impl MutationObserverInit {
    ///Construct a new `MutationObserverInit`.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `animations` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn animations(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("animations"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `attributeFilter` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributeFilter"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `attributeOldValue` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributeOldValue"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `attributes` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn attributes(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributes"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `characterData` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn character_data(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("characterData"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `characterDataOldValue` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("characterDataOldValue"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `childList` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn child_list(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("childList"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `nativeAnonymousChildList` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("nativeAnonymousChildList"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `subtree` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `MutationObserverInit`*

    pub fn subtree(&mut self, val: bool) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("subtree"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
