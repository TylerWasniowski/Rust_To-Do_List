extern crate regex;

pub mod db;
pub mod to_do_list;

#[cfg(test)]
mod to_do_list_tests {
    use to_do_list::ToDoList;
    use regex::Regex;

    #[test]
    fn position_first() {
        let to_do_list = setup_to_do_list();

        assert_eq!(to_do_list.position_first(Regex::new(r"(?i)laundry").unwrap()),
                   Some(0));
        assert_eq!(to_do_list.position_first(Regex::new(r"Foo").unwrap()),
                   None);
    }

    #[test]
    fn position_all() {
        let to_do_list = setup_to_do_list();

        assert_eq!(to_do_list.position_all(Regex::new(r"(?i)finish").unwrap()),
                   vec![1, 2]);
        assert_eq!(to_do_list.position_all(Regex::new(r"Foo").unwrap()),
                   Vec::new() as Vec<usize>);
    }

    #[test]
    fn mark() {
        let mut to_do_list = setup_to_do_list();

        to_do_list.mark(0, true);

        assert_eq!(to_do_list.to_string().contains("Do laundry: completed"), true);
    }

    #[test]
    fn rename() {
        let mut to_do_list = setup_to_do_list();

        to_do_list.rename(1, "Finish Math 179 HW".to_string());

        assert_eq!(to_do_list.to_string().contains("Finish Math 179 HW: not completed"), true);
    }

    #[test]
    fn remove() {
        let mut to_do_list = setup_to_do_list();

        to_do_list.remove(1);
        to_do_list.remove(2);
        to_do_list.remove(1);

        assert_eq!(to_do_list.to_string(), "Do laundry: not completed");
    }

    fn setup_to_do_list() -> ToDoList {
        let mut to_do_list = ToDoList::new();

        to_do_list.add("Do laundry".to_string());
        to_do_list.add("Finish homework".to_string());
        to_do_list.add("Finish to-do list".to_string());

        to_do_list
    }
}

#[cfg(test)]
mod database_tests {
    use db::Database;

    #[test]
    fn position_first() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();

        assert_eq!(empty_database.position_first(|value| value.len() < 5),
                   None);
        assert_eq!(database.position_first(|value| value.len() < 5),
                   Some(1));
    }

    #[test]
    fn position_all() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();

        assert_eq!(empty_database.position_all(|value| value.len() < 5),
                   Vec::new());
        assert_eq!(database.position_all(|value| value.len() < 5),
                   vec![1, 2, 5]);
    }

    #[test]
    fn find_first() {
        let empty_database: Database<String> = Database::new();
        let database = setup_database();

        assert_eq!(empty_database.find_first(|value| value.len() < 5),
                   None);
        assert_eq!(database.find_first(|value| value.len() < 5),
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

    #[test]
    fn remove_first() {
        let mut database: Database<String> = Database::new();
        database.add("Hello world!".to_string());
        database.add("Hi".to_string());

        database.remove_first(|value| value.len() < 5);
        database.remove_first(|value| value.len() < 5);

        assert_eq!(database.read_all(), &vec!["Hello world!".to_string()]);
    }

    #[test]
    fn remove_all() {
        let mut database = setup_database();

        database.remove_first(|value| value.len() < 5);
        database.remove_first(|value| value.len() < 5);
        database.remove_first(|value| value.len() < 5);
        database.remove_first(|value| value.len() < 5);

        assert_eq!(database.read_all(),
                   &vec!["Hello world!".to_string(), "Hello".to_string(), "Hola mundo!".to_string()]);
    }

    #[test]
    fn replace_first() {
        let mut database = setup_database();

        database.replace_first(|value| value == &&"Hello".to_string(),
                               "World".to_string());

        assert_eq!(database.get(3), Some(&"World".to_string()));
    }

    #[test]
    fn replace_all() {
        let mut database = setup_database();

        database.replace_all(|value| value.len() <= 5,
                               "World".to_string());
        database.replace_all(|value| value.len() > 50,
                             "Goodbye".to_string());

        assert_eq!(database.read_all(),
                   &vec!["Hello world!".to_string(), "World".to_string(), "World".to_string(),
                         "World".to_string(), "Hola mundo!".to_string(), "World".to_string()]);
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