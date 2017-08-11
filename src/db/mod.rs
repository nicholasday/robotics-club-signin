use rusqlite::Connection;
use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use types::Error;

pub mod member;
pub mod signin;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = SqliteConnectionManager::new(Path::new("data.db"));
    r2d2::Pool::new(config, manager).expect("Pool couldn't be made.")
}

pub struct Conn(pub r2d2::PooledConnection<SqliteConnectionManager>);

impl Deref for Conn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

type QueryResult<T> = Result<T, Error>;