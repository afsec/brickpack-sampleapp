use super::{InternalMessage, ShowUsers};

use brickpack::endpoint::{Name,View};

use tide::{http::mime, Response, StatusCode, Error as TideError};
use serde_json::to_string as serde_json_to_string;


impl View<InternalMessage> for ShowUsers {
    fn view(&self, result: Result<InternalMessage, TideError>) -> Response {
        let mut response = Response::builder(StatusCode::Ok)
            .content_type(mime::JSON)
            .build();
        match result {
            Ok(outcome) => {
                let json_body = serde_json_to_string(&outcome).unwrap_or("".to_owned());
                response.set_body(json_body);
            }
            Err(error) => {
                tide::log::error!(r#"Endpoint [{}]: {}"#, self.name(), error);
                #[cfg(debug_assertions)]
                response.insert_header("internal-error", error.to_string());

                response.set_body("[]");
            }
        }
        response

    }
}
