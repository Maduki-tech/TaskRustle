#![allow(dead_code)]
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Todo {
    id: i32,
    name: String,
    time: String,
}

#[derive(Debug, Clone)]
pub struct SqlDatabase {
    connection_string: String,
}

impl SqlDatabase {
    pub fn new(connection_string: String) -> SqlDatabase {
        SqlDatabase { connection_string }
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let conn = rusqlite::Connection::open(self.connection_string.clone()).unwrap();

        let mut todo = conn.prepare("SELECT * FROM todo").unwrap();

        let todo_iter = todo
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    time: row.get(2).unwrap(),
                })
            })
            .unwrap();

        let mut todos = Vec::new();

        for t in todo_iter {
            todos.push(t.unwrap());
        }

        todos
    }

    pub fn add_todo(&self, name: String, time: Option<String>) {
        let conn = rusqlite::Connection::open(self.connection_string.clone()).unwrap();

        let time = match time {
            Some(t) => t,
            None => DateTime::<Utc>::from(Utc::now()).to_rfc3339().to_string(),
        };

        conn.execute(
            "INSERT INTO todo (name, time) VALUES (?1, ?2)",
            &[&name, &time],
        )
        .unwrap();
    }
}
