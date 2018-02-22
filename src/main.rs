extern crate regex;

pub mod db;
pub mod to_do_list;

use to_do_list::ToDoList;

use regex::Regex;

fn main() {
    // Database test
    // TODO: Move to an actual tester
//    println!("Removing first string with length less than 5.");
//    database.remove_first(|data| data.len() < 5);
//    println!("Database: {:?}", database.read_all());
//    println!();
//    println!("Removing first string with length less than 5.");
//    database.remove_first(|data| data.len() < 5);
//    println!("Database: {:?}", database.read_all());
//    println!();
//    println!("Replacing second value with world.");
//    database.replace(1, "World".to_string());
//    println!("Database: {:?}", database.read_all());
//    println!();
//    println!("Replacing first string with length equal to 5 with 'Hello'.");
//    database.replace_first(|data| data.len() == 5,
//                           "Hello".to_string());
//    println!("Database: {:?}", database.read_all());
//    println!();
//    println!("Replacing all strings with length less than or equal to 5 with 'Goodbye'.");
//    database.replace_all(|data| data.len() <= 5,
//                           "Goodbye".to_string());
//    println!("Database: {:?}", database.read_all());
//    println!();
//    println!("Removing all strings with length less than 8.");
//    database.remove_all(|value| value.len() < 8);
//    println!("Database: {:?}", database.read_all());

    // ToDoList test
    let mut to_do_list = ToDoList::new();
    to_do_list.add("Do laundry".to_string());
    to_do_list.add("Finish homework".to_string());
    to_do_list.add("Finish to-do list".to_string());
    println!("ToDoList:");
    println!("{}", to_do_list);
    println!();
    println!("Looking for position of first entry containing laundry (case-insensitive): {:?}",
             to_do_list.position_first(Regex::new(r"(?i)laundry").unwrap()));
    println!();
    println!("Looking for all positions of entries containing Finish (case-insensitive): {:?}",
             to_do_list.position_all(Regex::new(r"(?i)finish").unwrap()));
    println!();
    println!("Marking first as completed.");
    to_do_list.mark(0, true);
    println!("ToDoList:");
    println!("{}", to_do_list);
}

#[cfg(test)]
mod database_tests {
    use db::Database;

    #[test]
    fn position_first() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();
        assert_eq!(empty_database.position_first(|data| data.len() < 5),
                   None);
        assert_eq!(database.position_first(|data| data.len() < 5),
                   Some(1));
    }

    #[test]
    fn position_all() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();
        assert_eq!(empty_database.position_all(|data| data.len() < 5),
                   Vec::new());
        assert_eq!(database.position_all(|value| value.len() < 5),
                   vec![1, 2, 5]);
    }

    #[test]
    fn find_first() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();
        assert_eq!(empty_database.find_first(|data| data.len() < 5),
                   None);
        assert_eq!(database.find_first(|data| data.len() < 5),
                   Some(&"Hi".to_string()));
    }

    #[test]
    fn find_all() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();
        assert_eq!(empty_database.find_all(|value| value.len() < 5),
            Vec::new() as Vec<&String>);
        assert_eq!(database.find_all(|value| value.len() > 5),
                   vec![&"Hello world!".to_string(), &"Hola mundo!".to_string()]);
        assert_eq!(database.find_all(|value| value == &&"Hello".to_string()),
                   vec![&"Hello".to_string()]);
        assert_eq!(database.find_all(|value| value.len() < 5),
                   vec![&"Hi".to_string(), &"Hey".to_string(), &"Yo".to_string()]);
    }

    fn setup_database() -> Database<String> {
        let mut database: Database<String> = Database::new();
        database.add("Hello world!".to_string());
        database.add("Hi".to_string());
        database.add("Hey".to_string());
        database.add("Hello".to_string());
        database.add("Hola mundo!".to_string());
        database.add("Yo".to_string());

        database
    }
}