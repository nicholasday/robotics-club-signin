#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate rocket;

extern crate chrono;
extern crate chrono_tz;
extern crate rocket_cors;
extern crate tera;

mod db;
mod members;
mod signins;
mod types;

use crate::members::*;
use crate::signins::*;

#[launch]
fn rocket() -> _ {
    let default = rocket_cors::CorsOptions {
        max_age: Some(3600),
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .attach(default)
        .attach(db::Conn::fairing())
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
}
