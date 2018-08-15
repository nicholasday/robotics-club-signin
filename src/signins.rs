use chrono::TimeZone;
use chrono_tz::America::New_York;
use db::signin::Signin;
use db::Conn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;
use std::fs;
use tera::Context;
use types::JsonResult;

extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PizzaList {
    pizzas: Vec<String>,
}

#[post("/pizzalist", format = "application/json", data = "<pizzas>")]
fn post_pizza_list(pizzas: Json<PizzaList>) -> JsonResult {
    let mut context = Context::new();

    match fs::write("pizzalist.txt", serde_json::to_string(&pizzas.0).unwrap()) {
        Ok(_) => {
            context.add("status", &"success");
            Ok(Json(context))
        }
        Err(_) => {
            context.add("status", &"failure");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[get("/pizzalist", format = "application/json")]
fn get_pizza_list() -> JsonResult {
    let mut context = Context::new();

    match fs::read_to_string("pizzalist.txt") {
        Ok(list) => {
            context.add("result", &serde_json::from_str::<PizzaList>(&list).unwrap());
            Ok(Json(context))
        }
        Err(_) => {
            context.add("status", &"failure");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[get("/signins", format = "application/json")]
fn get_signins(conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Signin::get_today(&conn) {
        Ok(signins) => {
            context.add("signins", &signins);
            Ok(Json(context))
        }
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[get("/signins/<year>/<month>/<day>", format = "application/json")]
fn get_signins_date(year: i32, month: u32, day: u32, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Signin::get_date(New_York.ymd(year, month, day).and_hms(1, 1, 1), &conn) {
        Ok(dates) => {
            context.add("signins", &dates);
            Ok(Json(context))
        }
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}
