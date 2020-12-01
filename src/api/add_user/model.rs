use super::{AddUser, InternalMessage};

use brickpack::endpoint::Model;

impl Model<InternalMessage> for AddUser {
    fn model(&self, request_body: String) -> InternalMessage {
        drop(request_body);
        InternalMessage
    }
}
