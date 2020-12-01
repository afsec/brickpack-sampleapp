use super::{AddUser, InternalMessage};

use brickpack::endpoint::View;

use tide::{Body, Response, StatusCode};

use serde::Serialize;

#[derive(Serialize)]
struct ResponseBody {
    status: StatusCode,
}

impl View<InternalMessage> for AddUser {
    fn view(&self, outcome: InternalMessage) -> Response {
        let body_response = ResponseBody {
            status: StatusCode::Ok,
        };
        Response::builder(StatusCode::Ok)
            .body(Body::from_json(&body_response).unwrap())
            .build()
    }
}
