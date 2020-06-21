use crate::api::{DB_URL_ENV_VAR, DEFAULT_DB_URL};

use brickpack::{env_vars::get_token_from_env, global_state::State, http_client::http_client};

use brickpack::Request;

pub fn show_posts(req: Request<State>) -> Result<String, String> {
    // Request data from Concierge-db Server
    // To Run:
    // git https://github.com/afsec/concierge-db
    // cd concierge-dbg
    // make deploy
    brickpack::log::debug!("{:?}", req);

    let db_url = match get_token_from_env(DB_URL_ENV_VAR) {
        Some(url) => url,
        None => {
            let msg = format!("Environment variable {} not found", DB_URL_ENV_VAR);
            brickpack::log::warn!("{}", &msg);
            DEFAULT_DB_URL.to_string()
        }
    };

    let body_request = r#"{ "table" : "posts" }"#.to_string();

    let method = "POST".to_string();
    let url = format!("{}/api/read-all", db_url);
    http_client(method, url, Some(body_request))
}
