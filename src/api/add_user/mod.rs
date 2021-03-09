mod model;
mod view;

use brickpack::{
    build_presenter,
    endpoint::{Endpoint, Name, Outcome, Presenter},
};

use brickpack_derive::{Endpoint, Outcome};
use tide::prelude::Serialize;

// Endpoint definition
#[derive(Debug, Endpoint)]
#[endpoint_name = "add_user"]
struct AddUser;

// Outcome definition
#[derive(Debug, Outcome, Serialize)]
struct InternalMessage {
    id: usize,
}

build_presenter!(AddUser, InternalMessage);