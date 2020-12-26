use waserv::{response::response, router::Router, Params, Waserv};
use wasm_bindgen_test::*;
use web_sys::{Headers, Request, RequestInit, Response};

fn index(_request: Request, _params: Params) -> Response {
    let body = "Hello, World!".to_string();
    let headers = Headers::new().unwrap();
    headers
        .set("Content-Type", "text/html; charset=UTF-8")
        .unwrap();
    headers.set("Cache-Control", "no-cache").unwrap();
    response(body, headers, Some(200))
}

#[wasm_bindgen_test]
fn handle_index() {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    let mut ww = Waserv::new();
    let mut router = Router::new();
    router.get("/", Box::new(index));
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

#[wasm_bindgen_test]
fn handle_not_found() {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    let mut ww = Waserv::new();
    let mut router = Router::new();
    router.get("/", Box::new(index));
    ww.mount(router);
    let mut request_init = RequestInit::new();
    request_init.method("GET");
    let request = Request::new_with_str_and_init("http://localhost/404", &request_init);
    let status = match request {
        Ok(req) => {
            let resp = ww.handle(req);
            resp.status()
        }
        Err(_) => 500,
    };
    assert_eq!(status, 404)
}
