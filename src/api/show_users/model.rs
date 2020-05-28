use crate::api::get_token_from_env;
use crate::api::DB_URL_ENV_VAR;
use brickpack::http_client::http_client;
use brickpack::http_client::CLIENT_TOKEN_ENV_VAR;

pub fn show_users(body: Option<String>) -> Result<String, String> {
    // Request data from Concierge-db Server
    // https://github.com/afsec/concierge-db
    let method = "GET".to_string();
    let url = format!(
        "{}/api/users/read-all",
        DB_URL_ENV_VAR
    );
    let result = http_client(method, url, get_token_from_env(CLIENT_TOKEN_ENV_VAR), None);
    result
}
