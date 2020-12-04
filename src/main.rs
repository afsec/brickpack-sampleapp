#![warn(clippy::all)]

mod api;

use brickpack::app::App;
use brickpack_derive::App;

use std::mem;

use clap::{crate_authors, crate_description, App as ClapApp};
use tide::{Request, Response, StatusCode};

#[derive(App)]
struct MyApp;

#[async_std::main]
async fn main() -> tide::Result<()> {
    ClapApp::new(MyApp.name())
        .version(MyApp.version())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    let addr = "127.0.0.1";
    let port = "8080";

    let listen = format!("{}:{}", addr, port);
    assert_eq!(0, mem::size_of_val(&MyApp));
    tide::log::start();
    let mut app = tide::new();
    tide::log::info!("Starting App [{} v{}]:", MyApp.name(), MyApp.version());
    tide::log::info!(
        "Powered by {} v{}",
        MyApp.powered_desc(),
        MyApp.powered_ver()
    );
    app.at("/").get(index_page);
    app.at("/auth").get(check_auth);
    app.at("/maintenance").patch(maintenance_mode);
    app.at("/api/:endpoint").post(dispatcher);
    app.listen(listen).await?;
    Ok(())
}

async fn index_page(req: Request<()>) -> tide::Result {
    drop(req);
    // TODO
    // Ok(Response::new(StatusCode::ImATeapot)) // If is in Maintenance Mode
    Ok(Response::new(StatusCode::Found))
}

async fn check_auth(req: Request<()>) -> tide::Result {
    drop(req);
    // TODO
    Ok(Response::new(StatusCode::Accepted))
}

async fn maintenance_mode(req: Request<()>) -> tide::Result {
    drop(req);
    // TODO
    Ok(Response::new(StatusCode::Accepted))
}

async fn dispatcher(mut request: Request<()>) -> tide::Result {
    let endpoint = request.param("endpoint")?;
    dbg!(&endpoint);
    match endpoint {
        "show_users" => crate::api::show_users::handler(request.body_string().await?),
        "add_user" => crate::api::add_user::handler(request.body_string().await?),
        "del_user" => crate::api::del_user::handler(request.body_string().await?),
        _ => {
            tide::log::warn!("Not found: {}", endpoint);
            Ok(Response::new(StatusCode::NotFound))
        }
    }
}

// TODO: User Journey Map (https://uxplanet.org/a-beginners-guide-to-user-journey-mapping-bd914f4c517c)
