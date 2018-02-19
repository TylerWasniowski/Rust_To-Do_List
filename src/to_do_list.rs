use db::Database;

use std::fmt;

pub struct ToDoList {
    entries: Database<(String, bool)>,
}

impl ToDoList {
    pub fn add(&mut self, item: String) {
        self.entries.add((item, false));
    }

    pub fn read_all(&self) -> &Database<(String, bool)> {
        &self.entries
    }

    pub fn mark(&mut self, index: usize, completed: bool) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.1 = completed;
        }
    }

    pub fn rename(&mut self, index: usize, item_name: String) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.0 = item_name;
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.entries.remove(index);
    }

    pub fn new() -> ToDoList {
        ToDoList {
            entries: Database::new(),
        }
    }
}

impl fmt::Display for ToDoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = self.entries.read_all()
            .iter()
            .fold(String::new(),
                  |accumulator, entry|
                      format!("{}{}: {}\n", accumulator, entry.0,
                                  if entry.1 { &"completed" } else { &"not completed" })
            );

        writeln!(f, "{}", ret)
    }
}
