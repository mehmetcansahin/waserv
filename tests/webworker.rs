use serde_json::json;
use wasm_bindgen_test::*;
use web_sys::{Request, Response};
use webworker::{response::response_json, Params};

fn index(_request: Request, _params: Params) -> Response {
    let data = json!({
        "status": "OK"
    });
    response_json(data)
}

#[wasm_bindgen_test]
fn handle() {
    let mut ww = webworker::WebWorker::new();
    let mut router = webworker::router::Router::new();
    router.add("/GET/".to_string(), Box::new(index));
    ww.mount(router);
    // TODO: Request
    // ww.handle(request);
}
