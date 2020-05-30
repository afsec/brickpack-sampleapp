use brickpack::global_state::State;
use tide::{Request, Response};

pub fn handler(request: Option<Request<State>>) -> Response {
    tide::log::info!("Hander show-posts");
    let model = super::model::show_posts(request.unwrap());
    let view = super::view::show_posts(model);
    view
}
