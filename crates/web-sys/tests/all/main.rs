extern crate wasm_bindgen_test_project_builder as project_builder;
use project_builder::{project, Project};

mod event;
mod headers;
mod anchor_element;
mod body_element;
mod br_element;
mod button_element;
mod div_element;
mod html_element;
// TODO fix on taskcluster
// mod head_element;
mod html_html_element;
mod script_element;
mod style_element;
// TODO fix on taskcluster
//mod span_element;
mod response;
mod element;
mod history;
/*TODO tests for:
    web_sys::HtmlFontElement,
    web_sys::HtmlMenuItemElement,
    web_sys::HtmlSourceElement,
    web_sys::HtmlAreaElement,
    web_sys::HtmlFormElement,
    web_sys::HtmlMetaElement,
    web_sys::HtmlAudioElement,
    web_sys::HtmlFrameElement,
    web_sys::HtmlMeterElement,
    web_sys::HtmlBaseElement,
    web_sys::HtmlFrameSetElement,
    web_sys::HtmlModElement,
    web_sys::HtmlTableCaptionElement,
    web_sys::HtmlObjectElement,
    web_sys::HtmlTableCellElement,
    web_sys::HtmlHeadingElement,
    web_sys::HtmlOListElement,
    web_sys::HtmlTableColElement,
    web_sys::HtmlHRElement,
    web_sys::HtmlOptGroupElement,
    web_sys::HtmlTableElement,
    web_sys::HtmlCanvasElement,
    web_sys::HtmlOptionElement,
    web_sys::HtmlTableRowElement,
    web_sys::HtmlDataElement,
    web_sys::HtmlIFrameElement,
    web_sys::HtmlOutputElement,
    web_sys::HtmlTableSectionElement,
    web_sys::HtmlDataListElement,
    web_sys::HtmlImageElement,
    web_sys::HtmlParagraphElement,
    web_sys::HtmlTemplateElement,
    web_sys::HtmlDetailsElement,
    web_sys::HtmlInputElement,
    web_sys::HtmlParamElement,
    web_sys::HtmlTextAreaElement,
    web_sys::HtmlDialogElement,
    web_sys::HtmlLabelElement,
    web_sys::HtmlPictureElement,
    web_sys::HtmlTimeElement,
    web_sys::HtmlDirectoryElement,
    web_sys::HtmlLegendElement,
    web_sys::HtmlPreElement,
    web_sys::HtmlTitleElement,
    web_sys::HtmlLIElement,
    web_sys::HtmlProgressElement,
    web_sys::HtmlTrackElement,
    web_sys::HtmlDListElement,
    web_sys::HtmlLinkElement,
    web_sys::HtmlQuoteElement,
    web_sys::HtmlUListElement,
    web_sys::HtmlMapElement,
    web_sys::HtmlVideoElement,
    web_sys::HtmlEmbedElement,
    web_sys::HtmlMediaElement,
    web_sys::HtmlSelectElement,
    web_sys::HtmlFieldSetElement,
    web_sys::HtmlMenuElement,
    web_sys::HtmlSlotElement,
*/
mod xpath_result;

fn websys_project() -> Project {
    project()
        .add_local_dependency("web-sys", env!("CARGO_MANIFEST_DIR"))
        .headless(true)
        .clone()
}
