#[macro_use]
extern crate rocket;

pub mod api;
pub mod model;
pub mod web;

#[launch]
pub fn rocket() -> _ {
    let app_data = model::AppState::init();

    let mut rc = rocket::build().manage(app_data);

    // mount web views (/)
    rc = web::build_web(rc, "/");
    // mount the api (/api)
    rc = api::build_api(rc, "/api");

    rc
}
