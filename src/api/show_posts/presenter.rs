use brickpack::global_state::State;
use brickpack::{Request, Response};

use super::{model,view};

pub fn handler(request: Option<Request<State>>) -> Response {
    brickpack::log::info!("Hander show-posts");
    let model = model::show_posts(request.unwrap());
    view::show_posts(model)
}
