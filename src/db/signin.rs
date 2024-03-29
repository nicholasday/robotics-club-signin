use crate::db::QueryResult;

use crate::types::Error::{AlreadySignedin, HaventSignedin};

use chrono::{DateTime, Utc};
use chrono_tz::America::New_York;
use chrono_tz::Tz;
use rocket_sync_db_pools::rusqlite;
use rocket_sync_db_pools::rusqlite::{params, Row};

#[derive(Debug, Serialize)]
pub struct Signin {
    pub id: i64,
    pub date_in: DateTime<Utc>,
    pub date_out: Option<DateTime<Utc>>,
    pub member_id: i64,
    pub pizza: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostPizza {
    pub pizza: String,
}

impl Signin {
    pub fn map(row: &Row) -> Result<Signin, rusqlite::Error> {
        Ok(Signin {
            id: row.get(0)?,
            member_id: row.get(1)?,
            pizza: row.get(2)?,
            date_in: row.get(3)?,
            date_out: row.get(4)?,
        })
    }
    pub fn get(id: &i64, conn: &rusqlite::Connection) -> QueryResult<Signin> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let mut statement = conn.prepare("SELECT * FROM signins WHERE id=?1")?;
        let member = statement.query_row(&[id], |row| Signin::map(&row))?;

        Ok(member)
    }

    pub fn get_date(date: DateTime<Tz>, conn: &rusqlite::Connection) -> QueryResult<Vec<Signin>> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let mut statement = conn.prepare("SELECT * FROM signins where date(date_in) = ?1")?;
        let rows = statement.query_map(&[&date.format("%Y-%m-%d").to_string()], |row| {
            Signin::map(&row)
        })?;

        let mut signins = Vec::new();
        for signin in rows {
            signins.push(signin?)
        }
        Ok(signins)
    }

    pub fn today_exists(id: &i64, conn: &rusqlite::Connection) -> QueryResult<bool> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let mut statement =
            conn.prepare("SELECT * FROM signins WHERE date(date_in) = ?1 AND member_id = ?2")?;
        let now = Utc::now();
        let now_local = now.with_timezone(&New_York);
        let result = statement.exists(params![&now_local.format("%Y-%m-%d").to_string(), id])?;
        Ok(result)
    }

    pub fn get_today(conn: &rusqlite::Connection) -> QueryResult<Vec<Signin>> {
        let now = Utc::now();
        let now_local = now.with_timezone(&New_York);
        Ok(Signin::get_date(now_local, &conn)?)
    }

    pub fn signin(id: &i64, pizza: &str, conn: &rusqlite::Connection) -> QueryResult<Signin> {
        if Signin::today_exists(&id, &conn)? == false {
            let mut statement = conn
                .prepare("INSERT into signins (member_id, pizza, date_in) VALUES (?1, ?2, ?3)")?;
            let now = Utc::now();
            let local_now = now.with_timezone(&New_York);
            let id = statement.insert(params![
                id,
                &pizza,
                &local_now.format("%Y-%m-%dT%H:%M:%S%.f").to_string(),
            ])?;
            let signin = Signin::get(&id, &conn)?;
            Ok(signin)
        } else {
            Err(AlreadySignedin)
        }
    }

    pub fn signout(id: &i64, conn: &rusqlite::Connection) -> QueryResult<Signin> {
        if Signin::today_exists(&id, &conn)? == true {
            let now = Utc::now();
            let local_now = now.with_timezone(&New_York);
            conn.execute(
                "UPDATE signins set date_out = ?1 WHERE member_id = ?2 AND date_out IS NULL",
                params![&local_now.format("%Y-%m-%dT%H:%M:%S%.f").to_string(), id],
            )?;
            let signin = Signin::get(&id, &conn)?;
            Ok(signin)
        } else {
            Err(HaventSignedin)
        }
    }
}
