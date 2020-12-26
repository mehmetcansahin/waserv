//! Waserv is a simple web server for a worker optimize
//!
//! # Example
//! ```no_run
//! use web_sys::{Request, Response, Headers};
//! use waserv::{response::response, router::Router, Params, Waserv};
//!
//! fn index(_request: Request, _params: Params) -> Response {
//!     let body = "Hello, World!".to_string();
//!     let headers = Headers::new().unwrap();
//!     headers
//!         .set("Content-Type", "text/html; charset=UTF-8")
//!         .unwrap();
//!     headers.set("Cache-Control", "no-cache").unwrap();
//!     response(body, headers, Some(200))
//! }
//!
//! fn handle(request: Request) -> Response {
//!     let mut router = Router::new();
//!     router.get("/", Box::new(index));
//!     let mut ww = Waserv::new();
//!     ww.mount(router);
//!     ww.handle(request)
//! }
//! ```
//!

use response::response;
use router::Router;
use web_sys::{Request, Response, Url};

#[macro_use]
pub mod macros;
pub mod response;
pub mod router;

pub type Params = Vec<(String, String)>;
type Handler = Box<dyn Fn(Request, Params) -> Response>;

pub struct Waserv {
    router: Router,
}

impl Waserv {
    pub fn new() -> Waserv {
        Waserv {
            router: Router::new(),
        }
    }

    pub fn handle(&self, request: Request) -> Response {
        let url = Url::new(&request.url()).unwrap();
        let path = format!("/{}{}", request.method().as_str(), &url.pathname());
        match self.router.tree.find(&path) {
            Some((func, params)) => {
                let params = params
                    .iter()
                    .map(|p| (p.0.to_string(), p.1.to_string()))
                    .collect::<Params>();
                func(request, params)
            }
            None => {
                let body = "404 - Not Found".to_string();
                let headers = headers! {
                    "Content-Type" => "text/html; charset=UTF-8",
                    "Cache-Control" => "no-cache"
                };
                response(body, headers, Some(404))
            }
        }
    }

    pub fn mount(&mut self, router: Router) {
        self.router = router;
    }
}
