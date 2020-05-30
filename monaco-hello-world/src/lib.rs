use monaco::editor::IStandaloneEditorConstructionOptions;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{console::log_1, HtmlElement};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    log_1(&JsValue::from_str("main_js"));

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let container: HtmlElement = JsCast::unchecked_into(document.get_element_by_id("container").expect("container"));
    let options = IStandaloneEditorConstructionOptions::new();
    options.set_value(Some("function x() {\n  console.log(\"Hello world!\");\n}"));
    options.set_language(Some("javascript"));
    monaco::editor::create(&container, Some(&options), None);
    Ok(())
}

pub mod monaco {
    pub mod editor {
        use js_sys::Object;
        use wasm_bindgen::{prelude::*, JsCast};
        use web_sys::HtmlElement;
        type HTMLElement = HtmlElement;

        #[wasm_bindgen(raw_module = "monaco-editor/esm/vs/editor/editor.main.js")]
        extern "C" {

            pub type IStandaloneEditorConstructionOptions;

            pub type IEditorOverrideServices;

            pub type IStandaloneCodeEditor;

            #[wasm_bindgen(method, setter, js_name=value)]
            pub fn set_value(this: &IStandaloneEditorConstructionOptions, value: Option<&str>);

            #[wasm_bindgen(method, setter, js_name=language)]
            pub fn set_language(this: &IStandaloneEditorConstructionOptions, value: Option<&str>);

            #[wasm_bindgen(js_namespace = editor)]
            pub fn create(
                dom_elmeent: &HTMLElement,
                options: Option<&IStandaloneEditorConstructionOptions>,
                override_: Option<&IEditorOverrideServices>,
            ) -> IStandaloneCodeEditor;
        }

        impl IStandaloneEditorConstructionOptions {
            pub fn new() -> Self {
                JsCast::unchecked_into(Object::new())
            }
        }
    }
}
