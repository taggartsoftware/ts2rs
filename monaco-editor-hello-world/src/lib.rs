mod monaco;
use monaco::editor::IStandaloneEditorConstructionOptions;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::HtmlElement;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // configure Monaco environment by setting the get_worker_url callback
    let monaco_environment = monaco::Environment::new();
    let get_worker_url = move |_worker_id: String, label: String| match label.as_str() {
        "json" => "./json.worker.js".to_owned(),
        "css" => "./css.worker.js".to_owned(),
        "html" => "./html.worker.js".to_owned(),
        "typescript" | "javascript" => "./ts.worker.js".to_owned(),
        _ => "./editor.worker.js".to_owned(),
    };
    monaco_environment.set_get_worker_url(&Closure::wrap(Box::new(get_worker_url)));
    let global = js_sys::global();
    js_sys::Object::define_property(&global, &JsValue::from_str("MonacoEnvironment"), monaco_environment.unchecked_ref());

    let window = web_sys::window().expect("window");
    let document = window.document().expect("document");
    let container: HtmlElement = document.get_element_by_id("container").expect("container").unchecked_into();
    let options = IStandaloneEditorConstructionOptions::new();
    options.set_value(Some("function x() {\n  console.log(\"Hello world!\");\n}"));
    options.set_language(Some("javascript"));
    monaco::editor::create(&container, Some(&options), None);

    Ok(())
}
