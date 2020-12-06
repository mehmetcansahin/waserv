use serde::Serialize;
use web_sys::{Headers, Response, ResponseInit};

pub fn response(body: String, headers: Headers, status: Option<u16>) -> Response {
    Response::new_with_opt_str_and_init(
        Some(&body),
        ResponseInit::new()
            .headers(headers.as_ref())
            .status(status.ok_or_else(|| 200).unwrap()),
    )
    .unwrap()
}

pub fn response_json<T>(data: T) -> Response
where
    T: Serialize,
{
    let body = serde_json::to_string(&data).unwrap();
    let headers = headers! {
        "Content-Type" => "application/json",
        "Cache-Control" => "no-cache"
    };
    response(body, headers, None)
}
