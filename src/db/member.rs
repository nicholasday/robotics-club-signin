use crate::db::QueryResult;

use rocket_sync_db_pools::rusqlite;
use rocket_sync_db_pools::rusqlite::{params, Row};

#[derive(Debug, Serialize)]
pub struct Member {
    id: i64,
    name: String,
    team: i64,
    last_pizza: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMember {
    name: String,
    team: i64,
    last_pizza: String,
}

impl Member {
    pub fn map(row: &Row) -> Result<Member, rusqlite::Error> {
        Ok(Member {
            id: row.get(0)?,
            name: row.get(1)?,
            team: row.get(2)?,
            last_pizza: row.get(3)?,
        })
    }
    pub fn all(conn: &rusqlite::Connection) -> QueryResult<Vec<Member>> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let mut statement = conn.prepare("SELECT * FROM members ORDER BY UPPER(name)")?;
        let rows = statement.query_map([], |row| Member::map(&row))?;

        let mut members = Vec::new();
        for member in rows {
            members.push(member?);
        }

        Ok(members)
    }

    pub fn get(id: &i64, conn: &rusqlite::Connection) -> QueryResult<Member> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let mut statement = conn.prepare("SELECT * FROM members WHERE id=?1")?;
        let member = statement.query_row(&[id], |row| Member::map(&row))?;

        Ok(member)
    }

    pub fn delete(id: &i64, conn: &rusqlite::Connection) -> QueryResult<Member> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let member = Member::get(id, &conn)?;
        conn.execute("DELETE FROM members WHERE id=?1", &[id])?;

        Ok(member)
    }

    pub fn signin(id: &i64, pizza: &str, conn: &rusqlite::Connection) -> QueryResult<Member> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        conn.execute(
            "UPDATE members SET last_pizza = ?1 WHERE id = ?2",
            params![&pizza, id],
        )?;
        let member = Member::get(id, &conn)?;

        Ok(member)
    }

    pub fn insert(member: &PostMember, conn: &rusqlite::Connection) -> QueryResult<Member> {
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        let mut statement =
            conn.prepare("INSERT into members (name, team, last_pizza) VALUES (?1, ?2, ?3)")?;
        let id = statement.insert(params![&member.name, &member.team, &member.last_pizza])?;
        let member = Member::get(&id, &conn)?;

        Ok(member)
    }
}
