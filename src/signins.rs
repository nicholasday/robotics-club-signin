use types::JsonResult;
use tera::Context;
use db::Conn;
use rocket_contrib::Json;
use db::signin::Signin;
use rocket::response::status::Custom;
use rocket::http::Status;

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