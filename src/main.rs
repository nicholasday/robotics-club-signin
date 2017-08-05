#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;

use std::path::Path;
use rusqlite::Connection;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]
fn create() -> &'static str {
    let path = Path::new("data.db");
    let conn = Connection::open(path).unwrap();
    "Hello, world!"
}

#[get("/insert")]
fn insert() -> &'static str {
    let path = Path::new("data.db");
    let conn = Connection::open(path).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data)
                  VALUES (?1, ?2)",
                 &[&me.name, &me.data]).unwrap();
    "Hello, world!"
}

#[get("/list")]
fn list() -> &'static str {
    let path = Path::new("data.db");
    let conn = Connection::open(path).unwrap();
        let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![insert, list, create, index]).launch();
}
