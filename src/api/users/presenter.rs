use http_types::{Response, StatusCode};
pub fn handler(body: String) -> Response {
    println!("Handler users ok!");
    let mut response = Response::new(StatusCode::Ok);
    response.set_body(body);
    response
}