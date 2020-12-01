use super::{AddUser, InternalMessage};

use brickpack::endpoint::Model;

use serde::Deserialize;
use serde_json::from_str;
use tide::StatusCode;

#[derive(Deserialize, Default)]
struct RequestBody {
    query: Option<String>,
}

impl Model<InternalMessage> for AddUser {
    fn model(&self, request_body: String) -> InternalMessage {
        let response_body: RequestBody = from_str(&request_body).unwrap_or_default();
        InternalMessage
    }
}
