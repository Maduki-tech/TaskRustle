use chrono::{DateTime, Local};
use rusqlite::Connection;

#[derive(Debug)]
struct Todo {
    id: i32,
    name: String,
    time: String
}


fn main(){
    let conn = Connection::open("database.db").unwrap();

    let mut todo = conn.prepare("SELECT * FROM todo").unwrap();

    let todo_iter = todo.query_map([], |row| {
        Ok(Todo{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            time: row.get(2).unwrap()

        })
    }).unwrap();

    for t in todo_iter {
        println!("{:?}", t.unwrap());
    }

}
