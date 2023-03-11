use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;
use web_sys::{Blob, BlobPropertyBag, Url};

// This is a not-so-clean approach to get the current bindgen ES module URL
// in Rust. This will fail at run time on bindgen targets not using ES modules.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    type ImportMeta;

    #[wasm_bindgen(method, getter)]
    fn url(this: &ImportMeta) -> JsString;

    #[wasm_bindgen(js_namespace = import, js_name = meta)]
    static IMPORT_META: ImportMeta;
}

pub fn on_the_fly(code: &str) -> Result<String, JsValue> {
    // Generate the import of the bindgen ES module, assuming `--target web`.
    let header = format!(
        "import init, * as bindgen from '{}';\n\n",
        IMPORT_META.url(),
    );

    Url::create_object_url_with_blob(&Blob::new_with_str_sequence_and_options(
        &Array::of2(&JsValue::from(header.as_str()), &JsValue::from(code)),
        &BlobPropertyBag::new().type_("text/javascript"),
    )?)
}

// dependent_module! takes a local file name to a JS module as input and
// returns a URL to a slightly modified module in run time. This modified module
// has an additional import statement in the header that imports the current
// bindgen JS module under the `bindgen` alias, and the separate init function.
// How this URL is produced does not matter for the macro user. on_the_fly
// creates a blob URL in run time. A better, more sophisticated solution
// would add wasm_bindgen support to put such a module in pkg/ during build time
// and return a URL to this file instead (described in #3019).
#[macro_export]
macro_rules! dependent_module {
    ($file_name:expr) => {
        crate::dependent_module::on_the_fly(include_str!($file_name))
    };
}
