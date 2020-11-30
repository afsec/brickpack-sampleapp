use super::{DelUser, InternalMessage};

use brickpack::endpoint::View;

use tide::{prelude::Serialize, Body, Response, StatusCode};

#[derive(Serialize)]
struct ResponseBody {
    status: StatusCode,
}

impl View<InternalMessage> for DelUser {
    fn view(&self, outcome: InternalMessage) -> Response {
        drop(outcome);
        let body_response = ResponseBody {
            status: StatusCode::Ok,
        };
        Response::builder(StatusCode::Ok)
            .body(Body::from_json(&body_response).unwrap())
            .build()
    }
}
