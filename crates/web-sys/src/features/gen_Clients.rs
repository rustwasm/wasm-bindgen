use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]

extern "C" {

    # [ wasm_bindgen ( extends = :: js_sys :: Object , js_name = Clients , typescript_type = "Clients" ) ]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The `Clients` class.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients)
    ///
    ///*This API requires the following crate features to be activated: `Clients`*
    pub type Clients;

    # [ wasm_bindgen ( method , structural , js_class = "Clients" , js_name = claim ) ]
    ///The `claim()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/claim)
    ///
    ///*This API requires the following crate features to be activated: `Clients`*
    pub fn claim(this: &Clients) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "Clients" , js_name = get ) ]
    ///The `get()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/get)
    ///
    ///*This API requires the following crate features to be activated: `Clients`*
    pub fn get(this: &Clients, id: &str) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "Clients" , js_name = matchAll ) ]
    ///The `matchAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)
    ///
    ///*This API requires the following crate features to be activated: `Clients`*
    pub fn match_all(this: &Clients) -> ::js_sys::Promise;

    #[cfg(feature = "ClientQueryOptions")]
    # [ wasm_bindgen ( method , structural , js_class = "Clients" , js_name = matchAll ) ]
    ///The `matchAll()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)
    ///
    ///*This API requires the following crate features to be activated: `ClientQueryOptions`, `Clients`*
    pub fn match_all_with_options(
        this: &Clients,
        options: &ClientQueryOptions,
    ) -> ::js_sys::Promise;

    # [ wasm_bindgen ( method , structural , js_class = "Clients" , js_name = openWindow ) ]
    ///The `openWindow()` method.
    ///
    ///[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clients/openWindow)
    ///
    ///*This API requires the following crate features to be activated: `Clients`*
    pub fn open_window(this: &Clients, url: &str) -> ::js_sys::Promise;

}
