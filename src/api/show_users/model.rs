use super::{InternalMessage, ShowUsers};

use brickpack::endpoint::Model;

impl Model<InternalMessage> for ShowUsers {
    fn model(&self, request_body: String) -> InternalMessage {
        drop(request_body);
        InternalMessage
    }
}
