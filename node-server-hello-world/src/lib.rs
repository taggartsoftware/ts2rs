mod logging;
use node::http::{create_server, IncomingMessage, RequestListener, ServerResponse};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    // based on https://nodejs.org/en/docs/guides/getting-started-guide/

    let default_port = 3000.;
    let port = match node::PROCESS.env().get("PORT") {
        Some(port) => port.parse::<f64>().unwrap_or(default_port),
        None => default_port,
    };

    let listener: RequestListener = Closure::wrap(Box::new(move |_req: IncomingMessage, res: ServerResponse| {
        res.set_status_code(200.);
        res.as_outgoing_message()
            .set_header("Content-Type", &JsValue::from_str("test-plain"));
        res.as_stream_writable().end(&JsValue::from_str("Hello World from WebAssembly"));
    }));

    let server = create_server(&listener);
    listener.forget();

    log!("server starting on port {}", port);
    server.as_net_net_server().listen(Some(port), None, None, &JsValue::UNDEFINED);
}
