use crate::api::{DB_URL_ENV_VAR, DEFAULT_DB_URL};

use brickpack::{
    env_vars::{get_token_from_env, CLIENT_TOKEN_ENV_VAR},
    global_state::State,
    http_client::http_client,
};

use tide::Request;

pub fn show_posts(req: Request<State>) -> Result<String, String> {
    dbg!(req);
    // Request data from Concierge-db Server
    // To Run:
    // git https://github.com/afsec/concierge-db
    // cd concierge-dbg
    // make deploy
    let db_url = match get_token_from_env(DB_URL_ENV_VAR) {
        Some(url) => url,
        None => {
            let msg = format!("Environment variable {} not found", DB_URL_ENV_VAR);
            tide::log::warn!("{}", &msg);
            DEFAULT_DB_URL.to_string()
        }
    };

    let token = match get_token_from_env(CLIENT_TOKEN_ENV_VAR) {
        Some(token) => token,
        None => {
            let msg = format!("Environment variable {} not found", CLIENT_TOKEN_ENV_VAR);
            tide::log::error!("{}", &msg);
            return Err(msg);
        }
    };

    let method = "GET".to_string();
    let url = format!("{}/api/posts/read-all", db_url);
    let result = http_client(method, url, Some(token), None);
    result
}
