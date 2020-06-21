mod logging;
use js_sys::Array;
use node::http::{create_server, IncomingMessage, RequestListener, ServerResponse};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(_argv: &Array) {
    // based on https://nodejs.org/en/docs/guides/getting-started-guide/

    let hostname = "127.0.0.1";
    let port = 3000.;

    let listener: RequestListener = Closure::wrap(Box::new(move |_req: IncomingMessage, res: ServerResponse| {
        res.set_status_code(200.);

        res.as_outgoing_message()
            .set_header("Content-Type", &JsValue::from_str("test-plain"));
        res.as_stream_writable().end(&JsValue::from_str("Hello World"));
    }));

    let server = create_server(&listener);
    listener.forget();

    server.as_net_server().listen(Some(port), Some(hostname), None, &JsValue::UNDEFINED);
}
