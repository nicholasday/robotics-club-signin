use crate::db::signin::Signin;
use crate::db::Conn;
use crate::types::JsonResult;
use chrono::TimeZone;
use chrono_tz::America::New_York;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use std::fs;
use tera::Context;

extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct PizzaList {
    pizzas: Vec<String>,
}

#[post("/pizzalist", format = "application/json", data = "<pizzas>")]
pub fn post_pizza_list(pizzas: Json<PizzaList>) -> JsonResult {
    let mut context = Context::new();

    match fs::write("pizzalist.txt", serde_json::to_string(&pizzas.0).unwrap()) {
        Ok(_) => {
            context.insert("status", &"success");
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("status", &"failure");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[get("/pizzalist", format = "application/json")]
pub fn get_pizza_list() -> JsonResult {
    let mut context = Context::new();

    match fs::read_to_string("pizzalist.txt") {
        Ok(list) => {
            context.insert("result", &serde_json::from_str::<PizzaList>(&list).unwrap());
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("status", &"failure");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[get("/signins", format = "application/json")]
pub async fn get_signins(conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let res = conn.run(move |c| Signin::get_today(&c)).await;
    match res {
        Ok(signins) => {
            context.insert("signins", &signins);
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[get("/signins/<year>/<month>/<day>", format = "application/json")]
pub async fn get_signins_date(year: i32, month: u32, day: u32, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let res = conn
        .run(move |c| Signin::get_date(New_York.ymd(year, month, day).and_hms(1, 1, 1), &c))
        .await;
    match res {
        Ok(dates) => {
            context.insert("signins", &dates);
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}
