use http_types::Response;
pub fn handler(body: Option<String>) -> Response {
    log::info!("Handler show-posts");
    let model = super::model::show_posts(body);
    let view = super::view::show_posts(model);
    view
}