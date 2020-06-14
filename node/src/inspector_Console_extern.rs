// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "inspector")]
extern "C" {
    #[doc = "Console message."]
    pub type ConsoleMessage;
    #[doc = "Message source."]
    #[wasm_bindgen(method, getter)]
    pub fn source(this: &ConsoleMessage) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_source(this: &ConsoleMessage, value: &str);
    #[doc = "Message severity."]
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &ConsoleMessage) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_level(this: &ConsoleMessage, value: &str);
    #[doc = "Message text."]
    #[wasm_bindgen(method, getter)]
    pub fn text(this: &ConsoleMessage) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_text(this: &ConsoleMessage, value: &str);
    #[doc = "URL of the message origin."]
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &ConsoleMessage) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_url(this: &ConsoleMessage, value: Option<&str>);
    #[doc = "Line number in the resource that generated this message (1-based)."]
    #[wasm_bindgen(method, getter)]
    pub fn line(this: &ConsoleMessage) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_line(this: &ConsoleMessage, value: Option<f64>);
    #[doc = "Column number in the resource that generated this message (1-based)."]
    #[wasm_bindgen(method, getter)]
    pub fn column(this: &ConsoleMessage) -> Option<f64>;
    #[wasm_bindgen(method, setter)]
    pub fn set_column(this: &ConsoleMessage, value: Option<f64>);
    pub type MessageAddedEventDataType;
    #[doc = "Console message that has been added."]
    #[wasm_bindgen(method, getter)]
    pub fn message(this: &MessageAddedEventDataType) -> inspector::console::ConsoleMessage;
    #[wasm_bindgen(method, setter)]
    pub fn set_message(
        this: &MessageAddedEventDataType,
        value: &inspector::console::ConsoleMessage,
    );
}