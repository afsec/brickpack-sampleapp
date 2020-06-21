use brickpack::global_state::State;
use brickpack::{Request, Response};

pub fn handler(request: Option<Request<State>>) -> Response {
    brickpack::log::info!("Hander show-posts");
    let model = super::model::show_posts(request.unwrap());
    let view = super::view::show_posts(model);
    view
}
