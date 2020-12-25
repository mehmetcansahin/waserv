# WebWorker - work in progress

[![crates.io](https://img.shields.io/crates/v/webworker.svg)](https://crates.io/crates/webworker)
[![Released API docs](https://docs.rs/webworker/badge.svg)](https://docs.rs/webworker)
[![GHA Build Status](https://github.com/mehmetcansahin/webworker/workflows/CI/badge.svg)](https://github.com/mehmetcansahin/webworker/actions?query=workflow%3ACI)

WebWorker is a simple web server for a wasm

Request -> CF Worker -> WebWorker -> Response

## Example

```rust
use web_sys::{Request, Response, Headers};
use webworker::{response::response, router::Router, Params, WebWorker};

fn index(_request: Request, _params: Params) -> Response {
    let body = "Hello, World!".to_string();
    let headers = Headers::new().unwrap();
    headers
        .set("Content-Type", "text/html; charset=UTF-8")
        .unwrap();
    headers.set("Cache-Control", "no-cache").unwrap();
    response(body, headers, Some(200))
}

fn handle(request: Request) -> Response {
    let mut router = Router::new();
    router.get("/", Box::new(index));
    let mut ww = WebWorker::new();
    ww.mount(router);
    ww.handle(request)
}
```

## Test

wasm-pack test --chrome
