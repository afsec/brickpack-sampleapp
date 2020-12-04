use super::{InternalMessage, DelUser};

use brickpack::endpoint::Model;
use tide::Error as TideError;

impl Model<InternalMessage> for DelUser {
    fn model(&self, request_body: String) -> Result<InternalMessage, TideError> {
        drop(request_body);
        Ok(InternalMessage(Vec::new()))
    }
}
