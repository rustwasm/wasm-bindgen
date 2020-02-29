use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = BrowserElementDownloadOptions ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `BrowserElementDownloadOptions` dictionary.
    ///
    ///*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*
    pub type BrowserElementDownloadOptions;

}

impl BrowserElementDownloadOptions {
    ///Construct a new `BrowserElementDownloadOptions`.
    ///
    ///*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*

    pub fn new() -> Self {

        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());

        ret
    }

    ///Change the `filename` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*

    pub fn filename(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("filename"),
            &JsValue::from(val),
        );

        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );

        let _ = r;

        self
    }

    ///Change the `referrer` field of this object.
    ///
    ///*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*

    pub fn referrer(&mut self, val: Option<&str>) -> &mut Self {

        use wasm_bindgen::JsValue;

        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("referrer"),
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
