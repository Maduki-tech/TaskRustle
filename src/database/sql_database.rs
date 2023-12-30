#[derive(Debug)]
pub struct Todo {
    id: i32,
    name: String,
    time: String
}

#[derive(Debug, Clone)]
pub struct SqlDatabase {
    connection_string: String,
}

impl SqlDatabase {
    pub fn new(connection_string: String) -> SqlDatabase {
        SqlDatabase{
            connection_string
        }
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let conn = rusqlite::Connection::open(self.connection_string.clone()).unwrap();

        let mut todo = conn.prepare("SELECT * FROM todo").unwrap();

        let todo_iter = todo.query_map([], |row| {
            Ok(Todo{
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                time: row.get(2).unwrap()

            })
        }).unwrap();

        let mut todos = Vec::new();

        for t in todo_iter {
            todos.push(t.unwrap());
        }

        todos
    }
}
