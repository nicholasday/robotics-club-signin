#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

extern crate chrono;
extern crate tera;

mod db;
mod cors;
mod members;
mod signins;
mod types;

use members::*;
use signins::*;
use db::init_pool;
use cors::CORS;

fn main() {
    rocket::ignite()
        .attach(CORS())
        .manage(init_pool())
        .mount("/", routes![signout_member, get_signins, signin_member, post_members, get_members])
        .launch();
}