use brickpack::{
    env_vars::get_token_from_env,
    global_state::State,
    http_client::http_client,
};

use tide::Request;

use crate::api::{DB_URL_ENV_VAR, DEFAULT_DB_URL};

pub fn show_users(req: Request<State>) -> Result<String, String> {
    // Request data from Concierge-db Server
    // https://github.com/afsec/concierge-db
    // dbg!(req);
    let uri = match get_token_from_env(DB_URL_ENV_VAR) {
        Some(uri) => uri,
        None => DEFAULT_DB_URL.to_string(),
    };
    let body_request = r#"{ "table" : "users" }"#.to_string();
    let method = "POST".to_string();
    let url = format!("{}/api/read-all", uri);
    let result = http_client(method, url, Some(body_request));
    result
}
