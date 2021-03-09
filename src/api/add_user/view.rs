use super::{AddUser, InternalMessage};

use brickpack::endpoint::{Name, View};

use serde_json::to_string as serde_json_to_string;
use tide::{http::mime, Error as TideError, Response, StatusCode,prelude::Serialize};

#[derive(Serialize)]
struct ResponseBody {
    status: String,
    description: String,
}

impl View<InternalMessage> for AddUser {
    fn view(&self, result: Result<InternalMessage, TideError>) -> Response {
        match result {
            Ok(outcome) => {
                tide::log::info!("ID created: {}", outcome.id);
                let json_body = serde_json_to_string(&outcome).unwrap_or("".to_owned());
                Response::builder(StatusCode::Ok)
                    .content_type(mime::JSON)
                    .body(json_body)
                    .build()
            }
            Err(error) => {
                tide::log::error!(r#"Endpoint [{}]: {}"#, self.name(), error);
                #[cfg(debug_assertions)]
                let error_message = error.to_string();


                let mut response = Response::from(error);
                response.set_content_type(mime::JSON);
                response.insert_header("internal-error", &error_message);
                let response_body = ResponseBody {
                    status: "Error".into(),
                    description: error_message,
                };
                let json_body = serde_json_to_string(&response_body).unwrap_or("".to_owned());
                response.set_body(json_body);
                response
            }
        }
    }
}
