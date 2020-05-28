use http_types::Response;
pub fn handler(body: Option<String>) -> Response {
    log::info!("Handler users ok!");
    let model = super::model::show_users(body);
    let view = super::view::show_users(model);
    view
}