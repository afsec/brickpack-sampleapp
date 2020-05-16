#![warn(clippy::all)]

use brickpack::{run, App};
use clap::{crate_authors, crate_description, crate_name, crate_version, App as ClapApp};

mod api;

fn main() {
    ClapApp::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let listen_addr = "0.0.0.0";
    let listen_port = "8000";

    let bind = format!("{}:{}", listen_addr, listen_port);

    let mut app = App::new();

    app.add_endpoint("crud-users", crate::api::users::presenter::handler);
    app.add_endpoint("crud-groups", crate::api::groups::presenter::handler);
    app.set_bind(bind);
    println!("Starting App [{} v{}]", crate_name!(), crate_version!());

    run(app).unwrap();
}
