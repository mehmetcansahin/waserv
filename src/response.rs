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
