#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

extern crate chrono;
extern crate chrono_tz;
extern crate tera;

mod cors;
mod db;
mod members;
mod signins;
mod types;

use cors::CORS;
use db::init_pool;
use members::*;
use signins::*;

fn main() {
    rocket::ignite()
        .attach(CORS())
        .manage(init_pool())
        .mount(
            "/",
            routes![
                get_pizza_list,
                post_pizza_list,
                delete_member,
                signout_member,
                get_signins_date,
                get_signins,
                signin_member,
                post_members,
                get_members
            ],
        )
        .launch();
}
