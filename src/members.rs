use types::JsonResult;
use tera::Context;
use db::Conn;
use rocket_contrib::Json;
use db::member::{Member, PostMember};
use db::signin::{Signin, PostPizza};
use rocket::response::status::Custom;
use rocket::http::Status;

#[get("/members", format = "application/json")]
fn get_members(conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Member::all(&conn) {
        Ok(members) => {
            context.add("members", &members);
            Ok(Json(context))
        },
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[get("/members/<id>/signout", format = "application/json")]
fn signout_member(id: i64, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Signin::signout(&id, &conn) {
        Ok(signin) => {
            context.add("signin", &signin);
            Ok(Json(context))
        },
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[post("/members/<id>/signin", format = "application/json", data = "<pizza>")]
fn signin_member(id: i64, pizza: Json<PostPizza>, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Signin::signin(&id, &(pizza.0).pizza, &conn) {
        Ok(signin) => {
            context.add("signin", &signin);
            match Member::signin(&id, &(pizza.0).pizza, &conn) {
                Ok(_) => {},
                Err(_) => {
                    context.add("error", &"Something went wrong.");
                    return Err(Custom(Status::BadRequest, Json(context)));
                }
            }
            Ok(Json(context))
        },
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}

#[post("/members", format = "application/json", data = "<member>")]
fn post_members(member: Json<PostMember>, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    match Member::insert(&member.0, &conn) {
        Ok(member) => {
            context.add("member", &member);
            Ok(Json(context))
        },
        Err(_) => {
            context.add("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, Json(context)))
        }
    }
}