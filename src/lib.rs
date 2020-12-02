use path_tree::PathTree;
use serde::Serialize;
use serde_json::json;
use web_sys::{ExtendableEvent, Headers, Request, Response, ResponseInit, Url};

#[macro_use]
pub mod macros;

pub type Params = Vec<(String, String)>;

pub struct WebWorker {
    pub(crate) tree: PathTree<Box<dyn Fn(Request, Params) -> Response>>,
}

impl Default for WebWorker {
    fn default() -> Self {
        let tree = PathTree::<Box<dyn Fn(Request, Params) -> Response>>::new();
        Self { tree: tree }
    }
}

impl WebWorker {
    pub fn route(&mut self, path: String, data: Box<dyn Fn(Request, Params) -> Response>) {
        self.tree.insert(&path, data);
    }

    pub fn handle(self, _event: ExtendableEvent, request: Request) -> Response {
        let url = Url::new(&request.url()).unwrap();
        let path = "/".to_owned() + request.method().as_str() + &url.pathname();
        match self.tree.find(&path) {
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
                response_json(data, Some(404))
            }
        }
    }
}

pub fn response(body: String, headers: Headers, status: Option<u16>) -> Response {
    Response::new_with_opt_str_and_init(
        Some(&body),
        ResponseInit::new()
            .headers(headers.as_ref())
            .status(status.ok_or_else(|| 200).unwrap()),
    )
    .unwrap()
}

pub fn response_json<T>(data: T, status: Option<u16>) -> Response
where
    T: Serialize,
{
    let body = serde_json::to_string(&data).unwrap();
    let headers = headers! {
        "Content-Type" => "application/json",
        "Cache-Control" => "no-cache"
    };
    response(body, headers, status)
}
