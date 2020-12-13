use response::response;
use router::Router;
use web_sys::{Request, Response, Url};

#[macro_use]
pub mod macros;
pub mod response;
pub mod router;

pub type Handler = Box<dyn Fn(Request, Params) -> Response>;
pub type Params = Vec<(String, String)>;

pub struct WebWorker {
    pub(crate) router: Router,
}

impl WebWorker {
    pub fn new() -> WebWorker {
        WebWorker {
            router: Router::new(),
        }
    }

    pub fn handle(&self, request: Request) -> Response {
        let url = Url::new(&request.url()).unwrap();
        let path = format!("/{}{}", request.method().as_str(), &url.pathname());
        match self.router.tree.find(&path) {
            Some((node, params)) => {
                let params = params
                    .iter()
                    .map(|p| (p.0.to_string(), p.1.to_string()))
                    .collect::<Params>();
                node(request, params)
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
