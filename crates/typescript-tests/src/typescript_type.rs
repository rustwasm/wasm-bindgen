use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
interface ITextStyle {
    bold: boolean;
    italic: boolean;
    size: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ITextStyle")]
    pub type ITextStyle;
}

#[wasm_bindgen]
#[derive(Default, Serialize, Deserialize)]
pub struct TextStyle {
    pub bold: bool,
    pub italic: bool,
    pub size: i32,
}

#[wasm_bindgen]
impl TextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(i: ITextStyle) -> TextStyle {
        let js_value: JsValue = i.into();
        serde_wasm_bindgen::from_value(js_value).unwrap()
    }

    pub fn optional_new(i: Option<ITextStyle>) -> TextStyle {
        i.map(Self::new).unwrap_or_default()
    }
}
