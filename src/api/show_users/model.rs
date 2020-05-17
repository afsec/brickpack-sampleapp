pub fn show_users(body: Option<String>) -> Result<String, String> {
    // let token = "9QYCfVgj5S7vJDxMtUDpca2AQdeliU9hCF5eVZK".to_string();
    // let url = "http://localhost:8000/api/crud-groups";
    //     let body_json = r#"{
    //     "user": "charlie.root",
    //     "password": "secret"
    // }"#;
    match body {
        Some(body) => Ok(body),
        None => Err("Empty Body".to_string()),
    }
}
