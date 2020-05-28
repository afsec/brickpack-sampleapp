pub mod show_posts;
pub mod show_users;

const DB_URL_ENV_VAR: &str = "DB_SERVER_URL";
const DEFAULT_DB_URL: &str = "http://concierge-db:3341";

fn get_token_from_env(token_name: &str) -> Option<String> {
    use std::env;
    match env::var(token_name) {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
