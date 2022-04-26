use crate::types::Error;
use rocket_sync_db_pools::{database, rusqlite::Connection};

pub mod member;
pub mod signin;

#[database("sqlite_db")]
pub struct Conn(pub Connection);

type QueryResult<T> = Result<T, Error>;
