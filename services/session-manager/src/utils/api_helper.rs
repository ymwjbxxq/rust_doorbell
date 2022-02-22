use lambda_http::{Response, http::StatusCode};

pub struct ApiHelper;

impl ApiHelper {
    pub fn response(status_code: StatusCode, body: String) -> Response<String> {
        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}