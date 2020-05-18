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

    let addr = "0.0.0.0";
    let port = "8000";

    let listen = format!("{}:{}", addr, port);

    let mut app = App::new();

    app.add_endpoint("show-users", crate::api::show_users::presenter::handler);
    app.add_endpoint("show-posts", crate::api::show_posts::presenter::handler);
    app.listen(listen);

    println!("Starting App [{} v{}]:", crate_name!(), crate_version!());

    run(app).unwrap();
}
