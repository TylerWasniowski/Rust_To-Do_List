pub mod db;
pub mod to_do_list;

use db::Database;

fn main() {
    let mut database: Database<String> = Database::new();
    database.add(String::from("Hello world!"));
    database.add(String::from("Hi"));
    database.add("Hey".to_string());
    database.add("Hello".to_string());
    database.add(String::from("Hola mundo!"));
    database.add("Yo".to_string());

    println!("Database: {:?}", database.read_all());
    println!("Find first string less than 5: {:?}",
             database.find_first(|data| data.len() < 5));
    println!("Find all strings less than 5: {:?}",
             database.find_all(|data| data.len() < 5));
    println!();
    println!("Removing first string with length less than 5.");
    database.remove_first(|data| data.len() < 5);
    println!("Database: {:?}", database.read_all());
    println!();
    println!("Removing first string with length less than 5.");
    database.remove_first(|data| data.len() < 5);
    println!("Database: {:?}", database.read_all());
    println!();
    println!("Replacing second value with world.");
    database.replace(1, "World".to_string());
    println!("Database: {:?}", database.read_all());
    println!();
    println!("Replacing first string with length equal to 5 with 'Hello'.");
    database.replace_first(|data| data.len() == 5,
                           "Hello".to_string());
    println!("Database: {:?}", database.read_all());
    println!();
    println!("Replacing all strings with length less than or equal to 5 with 'Goodbye'.");
    database.replace_all(|data| data.len() <= 5,
                           "Goodbye".to_string());
    println!("Database: {:?}", database.read_all());
    println!();
    println!("Removing all strings with length less than 8.");
    database.remove_all(|value| value.len() < 8);
    println!("Database: {:?}", database.read_all());
}