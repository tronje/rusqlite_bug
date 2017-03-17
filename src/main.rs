extern crate rusqlite;

use std::path::Path;
use rusqlite::Connection;

fn main() {
    // get a connection
    let conn = Connection::open(Path::new("demo.db")).unwrap();

    println!("running three queries to select the first password in the db...");
    println!("expected result for each would be 'password123'...\n");

    println!("query with two dynamic sql parameters");
    match conn.query_row::<String, _>(
        "SELECT ?1 FROM ?2",
        &[&"password", &"users"],
        |row| {
            row.get(0)
        }) {
        Ok(something) => println!("{}", something),
        Err(error) => println!("{:?}", error),
    }
    println!();

    println!("query with one dynamic sql parameter");
    match conn.query_row::<String, _>(
        "SELECT $1 FROM users;",
        &[&"password"],
        |row| {
            row.get(0)
        }) {
        Ok(something) => println!("{}", something),
        Err(error) => println!("{:?}", error),
    }
    println!();

    println!("query with no dynamic sql parameters");
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
