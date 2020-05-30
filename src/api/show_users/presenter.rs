use brickpack::global_state::State;
use tide::{Request, Response};

pub fn handler(request: Option<Request<State>>) -> Response {
    tide::log::info!("Handler users ok!");
    let model = super::model::show_users(request.unwrap());
    let view = super::view::show_users(model);
    view
}