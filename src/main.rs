mod database;
mod server;
// use database::sql_database::SqlDatabase;
use server::server::Server;

fn main(){
    // let db = SqlDatabase::new("database.db".to_string());
    //
    // let test = db.get_todos();
    //
    // test.iter().for_each(|t| {
    //     println!("{:?}", t);
    // });
    //
    // db.add_todo(
    //     "test".to_string(),
    //     Some("2021-01-01T00:00:00+00:00".to_string()),
    // );
    //
    // let test = db.get_todos();
    //
    // test.iter().for_each(|t| {
    //     println!("{:?}", t);
    // });

    let server = Server::new(8080);
    server.start();


}
