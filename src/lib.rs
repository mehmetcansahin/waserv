use response::response;
use router::Router;
use serde_json::json;
use web_sys::{Request, Response, Url};

#[macro_use]
pub mod macros;
pub mod response;
pub mod router;

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
        let path = "/".to_owned() + request.method().as_str() + &url.pathname();
        match self.router.tree.find(&path) {
            Some((node, params)) => {
                let params = params
                    .iter()
                    .map(|p| (p.0.to_string(), p.1.to_string()))
                    .collect::<Params>();
                node(request, params)
            }
            None => {
                let data = json!({
                    "status": "Not Found",
                    "code": 404
                });
                let body = serde_json::to_string(&data).unwrap();
                let headers = headers! {
                    "Content-Type" => "application/json",
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
