pub mod db;

use db::Database;

fn main() {
    let mut database: Database<String> = Database::new();
    database.add(String::from("Hello world!"));
    database.add(String::from("Hi"));
    println!("Database: {:?}", database.read_all());
}