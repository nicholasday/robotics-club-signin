use crate::db::member::{Member, PostMember};
use crate::db::signin::{PostPizza, Signin};
use crate::db::Conn;
use crate::types::JsonResult;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use tera::Context;

#[get("/members", format = "application/json")]
pub async fn get_members(conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let res = conn.run(move |c| Member::all(&c)).await;
    match res {
        Ok(members) => {
            context.insert("members", &members);
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[get("/members/<id>/signout", format = "application/json")]
pub async fn signout_member(id: i64, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let res = conn.run(move |c| Signin::signout(&id, &c)).await;
    match res {
        Ok(signin) => {
            context.insert("signin", &signin);
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[post("/members/<id>/signin", format = "application/json", data = "<pizza>")]
pub async fn signin_member(id: i64, pizza: Json<PostPizza>, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let clone_pizza = pizza.0.pizza.clone();
    let res = conn
        .run(move |c| Signin::signin(&id, &clone_pizza, &c))
        .await;
    match res {
        Ok(signin) => {
            context.insert("signin", &signin);
            let res = conn
                .run(move |c| Member::signin(&id, &(pizza.0).pizza, &c))
                .await;
            match res {
                Ok(_) => {}
                Err(_) => {
                    context.insert("error", &"Something went wrong.");
                    return Err(Custom(Status::BadRequest, context.into_json()));
                }
            }
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[delete("/member/<id>", format = "application/json")]
pub async fn delete_member(id: i64, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let res = conn.run(move |c| Member::delete(&id, &c)).await;
    match res {
        Ok(member) => {
            context.insert("member", &member);
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}

#[post("/members", format = "application/json", data = "<member>")]
pub async fn post_members(member: Json<PostMember>, conn: Conn) -> JsonResult {
    let mut context = Context::new();

    let res = conn.run(move |c| Member::insert(&member.0, &c)).await;
    match res {
        Ok(member) => {
            context.insert("member", &member);
            Ok(context.into_json())
        }
        Err(_) => {
            context.insert("error", &"Something went wrong.");
            Err(Custom(Status::BadRequest, context.into_json()))
        }
    }
}
