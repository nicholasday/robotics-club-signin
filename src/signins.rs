use types::JsonResult;
use tera::Context;
use db::Conn;
use rocket_contrib::Json;
use db::signin::Signin;
use rocket::response::status::Custom;
use rocket::http::Status;
use chrono::TimeZone;
use chrono_tz::America::New_York;

#[get("/signins", format = "application/json")]
fn get_signins(conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Signin::get_today(&conn) {
        Ok(signins) => {
            context.add("signins", &signins);
            Ok(Json(context))
        },
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[get("/signins/<year>/<month>/<day>", format = "application/json")]
fn get_signins_date(year: i32, month: u32, day: u32, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Signin::get_date(New_York.ymd(year, month, day), &conn) {
        Ok(dates) => {
            context.add("dates", &dates);
            Ok(Json(context))
        },
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }

}