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
#[endpoint_name = "show_users"]
struct ShowUsers;

// Outcome definition
#[derive(Debug, Outcome,Serialize)]
struct InternalMessage(Vec<String>);

build_presenter!(ShowUsers, InternalMessage);
