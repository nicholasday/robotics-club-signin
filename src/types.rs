use rusqlite;
use rocket::response::status::Custom;
use rocket_contrib::Json;
use tera::Context;

pub enum Error {
    SqliteError,
    AlreadySignedin,
    HaventSignedin
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Error {
        println!("{:?}", e);
        Error::SqliteError
    }
}

pub type JsonResult = Result<Json<Context>, Custom<Json<Context>>>;