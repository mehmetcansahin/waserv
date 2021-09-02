# Waserv - work in progress

[![crates.io](https://img.shields.io/crates/v/waserv.svg)](https://crates.io/crates/waserv)
[![Released API docs](https://docs.rs/waserv/badge.svg)](https://docs.rs/waserv)
[![GHA Build Status](https://github.com/mehmetcansahin/waserv/workflows/CI/badge.svg)](https://github.com/mehmetcansahin/waserv/actions?query=workflow%3ACI)

Waserv is a simple web server for a wasm

Request -> Cloudflare Worker -> Waserv -> Response

## Example

```rust
use web_sys::{Request, Response, Headers};
use waserv::{response::response, router::Router, Params, Waserv};

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
    let mut ww = Waserv::new();
    ww.mount(router);
    ww.handle(request)
}
```

## Test

wasm-pack test --chrome
