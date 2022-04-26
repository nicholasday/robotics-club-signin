use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket_sync_db_pools::rusqlite;

pub enum Error {
    SqliteError,
    AlreadySignedin,
    HaventSignedin,
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Error {
        println!("{:?}", e);
        Error::SqliteError
    }
}

pub type JsonResult = Result<Value, Custom<Value>>;
