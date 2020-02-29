use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = TreeCellInfo ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `TreeCellInfo` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `TreeCellInfo`*
    pub type TreeCellInfo;

}

impl TreeCellInfo {
    ///Construct a new `TreeCellInfo`.
    ///
    ///*This API requires the following crate features to be activated: `TreeCellInfo`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `childElt` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TreeCellInfo`*

    pub fn child_elt(&mut self, val: &str) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("childElt"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `row` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `TreeCellInfo`*

    pub fn row(&mut self, val: i32) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("row"), &JsValue::from(val));

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }
}
