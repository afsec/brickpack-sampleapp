use super::{AddUser, InternalMessage};

use brickpack::endpoint::Model;
use tide::{Error as TideError, StatusCode};

impl Model<InternalMessage> for AddUser {
    fn model(&self, request_body: String) -> Result<InternalMessage, TideError> {
        drop(request_body); //TODO: Launch a Database Query
        // let database_query_value: usize = 0; //TODO: Value from Database Query
        // Ok(InternalMessage {
        //     id: database_query_value,
        // })

        let msg = "ruSQLite low level error!";

        let error = tide::Error::from_str(StatusCode::NotAcceptable, msg);
        Err(error)
    }
}