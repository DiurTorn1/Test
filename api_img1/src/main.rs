#![feature(plugin, const_fn, decl_macro)]
//#![plugin(rocket_codegen)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;

mod models;
mod schema;
mod db;
mod static_files;
mod routes;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", 
            routes![routes::index, routes::new, routes::show,
            routes::delete, routes::nameimg, routes::update],
        )
        .mount("/", routes![static_files::all, static_files::index])
        //.catch(catchers![not_found])
}
fn main() {
    rocket().launch();
}
