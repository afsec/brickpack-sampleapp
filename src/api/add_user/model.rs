use super::{InternalMessage, AddUser};

use brickpack::endpoint::Model;
use tide::Error as TideError;

impl Model<InternalMessage> for AddUser {
    fn model(&self, request_body: String) -> Result<InternalMessage, TideError> {
        drop(request_body);
        Ok(InternalMessage(Vec::new()))
    }
}
