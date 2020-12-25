use web_sys::{Headers, Response, ResponseInit};

/// Response
pub fn response(body: String, headers: Headers, status: Option<u16>) -> Response {
    Response::new_with_opt_str_and_init(
        Some(&body),
        ResponseInit::new()
            .headers(headers.as_ref())
            .status(status.ok_or_else(|| 200).unwrap()),
    )
    .unwrap()
}

/// JSON Response
pub fn json(body: String) -> Response {
    let headers = headers! {
        "Content-Type" => "text/json",
        "Cache-Control" => "no-cache"
    };
    Response::new_with_opt_str_and_init(
        Some(&body),
        ResponseInit::new().headers(headers.as_ref()).status(200),
    )
    .unwrap()
}
