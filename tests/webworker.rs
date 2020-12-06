extern crate webworker;

use wasm_bindgen_test::*;
use web_sys::{Headers, Request, Response};
use webworker::{response::response, Params};

fn index(_request: Request, _params: Params) -> Response {
    let body = "404 - Not Found".to_string();
    let headers = Headers::new().unwrap();
    headers
        .set("Content-Type", "text/html; charset=UTF-8")
        .unwrap();
    headers.set("Cache-Control", "no-cache").unwrap();
    response(body, headers, Some(200))
}

#[wasm_bindgen_test]
fn handle() {
    let mut ww = webworker::WebWorker::new();
    let mut router = webworker::router::Router::new();
    router.add("/GET/".to_string(), Box::new(index));
    ww.mount(router);
    // TODO: Request
    // let request = Request::new_with_str("/").unwrap();
    // ww.handle(request);
}
