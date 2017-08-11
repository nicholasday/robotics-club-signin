use db::QueryResult;
use db::Conn;

use rusqlite::Row;

#[derive(Debug, Serialize)]
pub struct Member {
    id: i64,
    name: String,
    team: i64,
    last_pizza: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMember {
    name: String,
    team: i64,
    last_pizza: String
}

impl<'a> From<&'a &'a Row<'a, 'a>> for Member {
    fn from(row: &'a &'a Row<'a, 'a>) -> Member {
        Member {
            id: row.get(0),
            name: row.get(1),
            team: row.get(2),
            last_pizza: row.get(3),
        }
    }
}

impl Member {
    pub fn all(conn: &Conn) -> QueryResult<Vec<Member>> {
        let mut statement = conn.prepare("SELECT * FROM members ORDER BY UPPER(name)")?;
        let rows = statement.query_map(&[], |row| Member::from(&row))?;

        let mut members = Vec::new();
        for member in rows {
            members.push(member?);
        }

        Ok(members)
    }

    pub fn get(id: &i64, conn: &Conn) -> QueryResult<Member> {
        let mut statement = conn.prepare("SELECT * FROM members WHERE id=?1")?;
        let member = statement.query_row(&[id], |row| Member::from(&row))?;

        Ok(member)
    }

    pub fn signin(id: &i64, pizza: &str, conn: &Conn) -> QueryResult<Member> {
        conn.execute("UPDATE members SET last_pizza = ?1 WHERE id = ?2", &[&pizza, id])?;
        let member = Member::get(id, &conn)?;

        Ok(member)
    }

    pub fn insert(member: &PostMember, conn: &Conn) -> QueryResult<Member> {
        let mut statement = conn.prepare("INSERT into members (name, team, last_pizza) VALUES (?1, ?2, ?3)")?;
        let id = statement.insert(&[&member.name, &member.team, &member.last_pizza])?;
        let member = Member::get(&id, &conn)?;

        Ok(member)
    }
}
