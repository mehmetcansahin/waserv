use wasm_bindgen_test::*;
use web_sys::{Headers, Request, RequestInit, Response};
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
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    let mut ww = webworker::WebWorker::new();
    let mut router = webworker::router::Router::new();
    router.add("/GET/".to_string(), Box::new(index));
    ww.mount(router);
    let mut request_init = RequestInit::new();
    request_init.method("GET");
    let request = Request::new_with_str_and_init("http://localhost", &request_init);
    let status = match request {
        Ok(req) => {
            let resp = ww.handle(req);
            resp.status()
        }
        Err(_) => 500,
    };
    assert_eq!(status, 200)
}
