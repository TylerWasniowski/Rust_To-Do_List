
use db::Database;

use regex::Regex;

use std::fmt;

pub struct ToDoList {
    entries: Database<Entry>,
}

struct Entry {
    task: String,
    completed: bool,
}

impl ToDoList {
    pub fn add(&mut self, item: String) {
        self.entries.add(Entry::new(item));
    }

    pub fn position_first(&self, reg_exp: Regex) -> Option<usize> {
        self.entries.position_first(|entry| reg_exp.is_match(entry.get_task()))
    }

    pub fn position_all(&self, reg_exp: Regex) -> Vec<usize> {
        self.entries.position_all(|entry| reg_exp.is_match(entry.get_task()))
    }

    pub fn mark(&mut self, index: usize, completed: bool) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.mark(completed);
        }
    }

    pub fn rename(&mut self, index: usize, task: String) {
        if let Some(entry) = self.entries.get_mut(index) {
            entry.rename(task);
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
        let mut ret = self.entries.read_all()
            .iter()
            .fold(String::new(),
                  |accumulator, entry|
                      format!("{}{}\n", accumulator, entry)
            );

        // Remove trailing newline
        ret.pop();

        write!(f, "{}", ret)
    }
}

impl Entry {
    fn get_task(&self) -> &String {
        &self.task
    }

    fn is_completed(&self) -> bool {
        self.completed
    }

    fn mark(&mut self, completed: bool) {
        self.completed = completed;
    }

    fn rename(&mut self, task: String) {
        self.task = task;
    }

    fn new(task: String) -> Entry {
        Entry {
            task,
            completed: false,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.get_task(),
               if self.is_completed() { &"completed" } else { &"not completed" })
    }
}
