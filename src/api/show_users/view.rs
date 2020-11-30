use super::{InternalMessage, ShowUsers};

use brickpack::endpoint::View;

use tide::{prelude::Serialize, Body, Response, StatusCode};

#[derive(Serialize)]
struct ResponseBody {
    result: Vec<String>,
}

impl View<InternalMessage> for ShowUsers {
    fn view(&self, outcome: InternalMessage) -> Response {
        drop(outcome);
        let body_response = ResponseBody { result: Vec::new() };
        Response::builder(StatusCode::Ok)
            .body(Body::from_json(&body_response).unwrap())
            .build()
    }
}
