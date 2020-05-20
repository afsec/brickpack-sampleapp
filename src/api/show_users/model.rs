use brickpack::http_client::http_client;

pub fn show_users(body: Option<String>) -> Result<String, String> {
    let token = "9admin9".to_string();
    // Request data from Concierge-db Server
    // https://github.com/afsec/concierge-db
    let url = "http://localhost:3341/api/users/read-all".to_string();
    let result = http_client(url, Some(token), None);
    result
}
