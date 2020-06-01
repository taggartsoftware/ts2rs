use js_sys::Object;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen()]
extern "C" {

    pub type Environment;

    # [ wasm_bindgen ( method , setter , js_name = getWorkerUrl ) ]
    pub fn set_get_worker_url(this: &Environment, value: &Closure<dyn FnMut(String, String) -> String>);
}

impl Environment {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}

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
            Object::new().unchecked_into()
        }
    }
}
