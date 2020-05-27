use brickpack::http_client::http_client;

pub fn show_posts(body: Option<String>) -> Result<String, String> {
    // Request data from Concierge-db Server
    // To Run:
    // git https://github.com/afsec/concierge-db
    // cd concierge-dbg
    // make deploy
    let method = "GET".to_string();
    let url = "http://concierge-db:3341/api/posts/read-all".to_string();
    let token = "9admin9".to_string();
    let result = http_client(method, url, Some(token), None);
    result
}
