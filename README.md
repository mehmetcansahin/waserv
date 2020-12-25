# WebWorker - work in progress

[![crates.io](https://img.shields.io/crates/v/webworker.svg)](https://crates.io/crates/webworker)
[![Released API docs](https://docs.rs/webworker/badge.svg)](https://docs.rs/webworker)
[![GHA Build Status](https://github.com/mehmetcansahin/webworker/workflows/CI/badge.svg)](https://github.com/mehmetcansahin/webworker/actions?query=workflow%3ACI)

WebWorker is a simple web server for a wasm

Request -> CF Worker -> WebWorker -> Response

## Example

```rust
let mut router = Router::new();
router.get("/", Box::new(index));
let mut ww = WebWorker::new();
ww.mount(router);
ww.handle(request)
```

## Test

wasm-pack test --chrome
