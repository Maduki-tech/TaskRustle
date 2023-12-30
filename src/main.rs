mod database;
use database::sql_database::SqlDatabase;

fn main(){
    let db = SqlDatabase::new("database.db".to_string());

    let test = db.get_todos();

    test.iter().for_each(|t| {
        println!("{:?}", t);
    });

}
