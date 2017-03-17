extern crate rusqlite;

use std::path::Path;
use rusqlite::{Connection, Error};

fn main() {
    // get a connection
    let conn = Connection::open(Path::new("demo.db")).unwrap();

    match conn.query_row::<String, _>(
        "SELECT ?1 FROM ?2",
        &[&"password", &"users"],
        |row| {
            row.get(0)
        }) {
        Ok(something) => println!("{}", something),
        Err(error) => println!("{:?}", error),
    }

    match conn.query_row::<String, _>(
        "SELECT $1 FROM users;",
        &[&"password"],
        |row| {
            row.get(0)
        }) {
        Ok(something) => println!("{}", something),
        Err(error) => println!("{:?}", error),
    }

    match conn.query_row::<String, _>(
        "SELECT password FROM users;",
        &[],
        |row| {
            row.get(0)
        }) {
        Ok(something) => println!("{}", something),
        Err(error) => println!("{:?}", error),
    }
}
