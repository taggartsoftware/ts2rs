use wasm_bindgen::{prelude::*, JsValue};
use web_sys::console::log_1;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    log_1(&JsValue::from_str("Hello World!"));
    Ok(())
}

// pub mod monaco {
//     pub mod editor {
//         use js_sys::Object;
//         use wasm_bindgen::{prelude::*, JsCast};
//         use web_sys::HtmlElement;
//         type HTMLElement = HtmlElement;

//         #[wasm_bindgen(module = "monaco", js_namespace = ["monaco", "editor"])]
//         extern "C" {

//             pub type IStandaloneEditorConstructionOptions;

//             pub type IEditorOverrideServices;

//             pub type IStandaloneCodeEditor;

//             #[wasm_bindgen(method, setter, js_name=value)]
//             pub fn set_value(this: &IStandaloneEditorConstructionOptions, value: Option<&str>);

//             #[wasm_bindgen(method, setter, js_name=language)]
//             pub fn set_language(this: &IStandaloneEditorConstructionOptions, value: Option<&str>);

//             // export function create(domElement: HTMLElement, options?: IStandaloneEditorConstructionOptions, override?: IEditorOverrideServices): IStandaloneCodeEditor;
//             pub fn create(
//                 dom_elmeent: &HTMLElement,
//                 options: Option<&IStandaloneEditorConstructionOptions>,
//                 override_: Option<IEditorOverrideServices>,
//             ) -> IStandaloneCodeEditor;
//         }

//         impl IStandaloneEditorConstructionOptions {
//             pub fn new() -> Self {
//                 JsCast::unchecked_into(Object::new())
//             }
//         }
//     }
// }
