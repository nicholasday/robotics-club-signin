use db::QueryResult;
use db::Conn;

use types::Error::{AlreadySignedin, HaventSignedin};

use chrono::{Date, DateTime, Utc};
use rusqlite::Row;

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
    pub pizza: String
}

impl<'a> From<&'a &'a Row<'a, 'a>> for Signin {
    fn from(row: &'a &'a Row<'a, 'a>) -> Signin {
        Signin {
            id: row.get(0),
            member_id: row.get(1),
            pizza: row.get(2),
            date_in: row.get(3),
            date_out: row.get(4),
        }
    }
}

impl Signin {
    pub fn get(id: &i64, conn: &Conn) -> QueryResult<Signin> {
        let mut statement = conn.prepare("SELECT * FROM signins WHERE id=?1")?;
        let member = statement.query_row(&[id], |row| Signin::from(&row))?;

        Ok(member)
    }

    pub fn get_date(date: Date<Utc>, conn: &Conn) -> QueryResult<Vec<Signin>> {
        let mut statement = conn.prepare("SELECT * FROM signins where date(date_in) = ?1")?;
        let rows = statement.query_map(&[&date.format("%Y-%m-%d").to_string()], |row| Signin::from(&row))?;

        let mut signins = Vec::new();
        for signin in rows {
            signins.push(signin?)
        }
        Ok(signins)
    }

    pub fn today_exists(id: &i64, conn: &Conn) -> QueryResult<bool> {
        let mut statement = conn.prepare("SELECT * FROM signins WHERE date(date_in) = ?1 AND member_id = ?2")?;
        let result = statement.exists(&[&Utc::today().format("%Y-%m-%d").to_string(), id])?;

        Ok(result)
    }

    pub fn get_today(conn: &Conn) -> QueryResult<Vec<Signin>> {
        Ok(Signin::get_date(Utc::today(), &conn)?)
    }

    pub fn signin(id: &i64, pizza: &str, conn: &Conn) -> QueryResult<Signin> {
        if Signin::today_exists(&id, &conn)? == false {
            let mut statement = conn.prepare("INSERT into signins (member_id, pizza, date_in) VALUES (?1, ?2, ?3)")?;
            let id = statement.insert(&[id, &pizza, &Utc::now()])?;
            let signin = Signin::get(&id, &conn)?;
            Ok(signin)
        } else {
            Err(AlreadySignedin)
        }
    }

    pub fn signout(id: &i64, conn: &Conn) -> QueryResult<Signin> {
        if Signin::today_exists(&id, &conn)? == true {
            conn.execute("UPDATE signins set date_out = ?1 WHERE member_id = ?2 AND date_out IS NULL", &[&Utc::now(), id])?;
            let signin = Signin::get(&id, &conn)?;
            Ok(signin)
        } else {
            Err(HaventSignedin)
        }
    }

}