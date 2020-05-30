#![warn(clippy::all)]

mod api;

use clap::{crate_authors, crate_description, crate_name, crate_version, App as ClapApp};
use brickpack::App;


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

    app.set_name(crate_name!());
    app.set_version(crate_version!());

    app.set_listen(listen);

    app.run().unwrap();
}
