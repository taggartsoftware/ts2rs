// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "inspector")]
extern "C" {
    pub type NotifyWhenWaitingForDisconnectParameterType;
    #[wasm_bindgen(method, getter)]
    pub fn enabled(this: &NotifyWhenWaitingForDisconnectParameterType) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_enabled(this: &NotifyWhenWaitingForDisconnectParameterType, value: bool);
}
