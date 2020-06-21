use brickpack::{Response, StatusCode};

pub fn show_users(body: Result<String, String>) -> Response {
    match body {
        Ok(body_string) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(body_string);
            response
        }
        Err(body_string) => {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(body_string);
            response
        }
    }
}
