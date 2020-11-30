use super::{DelUser, InternalMessage};

use brickpack::endpoint::Model;

impl Model<InternalMessage> for DelUser {
    fn model(&self, request_body: String) -> InternalMessage {
        drop(request_body);
        InternalMessage
    }
}
