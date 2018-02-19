use std::ops::Fn;
use std::ops::FnMut;

#[derive(Debug)]
pub struct Database<T> {
    data: Vec<T>,
}

impl<T> Database<T> {
    pub fn add(&mut self, data: T) {
        self.data.push(data);
    }

    pub fn read_all(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.data.get(i)
    }

    pub fn find_first<F: Fn(&&T) -> bool>(&self, predicate: F) -> Option<&T> {
        self.data.iter()
            .find(predicate)
    }

    pub fn find_all<F: Fn(&&T) -> bool>(&self, predicate: F) -> Vec<&T> {
        self.data.iter()
            .filter(predicate)
            .collect::<Vec<&T>>()
    }

    pub fn replace(&mut self, i: usize, new_value: T) {
        if let Some(value) = self.data.get_mut(i) {
            *value = new_value;
        }
    }

    pub fn replace_first<F: FnMut(&&mut T) -> bool>(&mut self, predicate: F, new_value: T) {
        if let Some(value) = self.data.iter_mut()
            .find(predicate)
        {
            *value = new_value;
        }
    }

    pub fn replace_all<F>(&mut self, predicate: F, new_value: T)
        where F: Fn(&T) -> bool,
              T: Clone,
    {
        self.data.iter_mut()
            .for_each(|value| {
                if predicate(value) {
                    *value = new_value.clone();
                }
            });
    }

    pub fn remove_first<F: Fn(&T) -> bool>(&mut self, predicate: F) {
        if let Some(i) = self.data.iter()
            .position(predicate)
        {
            self.data.remove(i);
        }
    }

    pub fn remove_all<F: Fn(&T) -> bool>(&mut self, predicate: F) {
        self.data.retain(|data| !predicate(data));
    }

    pub fn new() -> Database<T> {
        Database {
            data: Vec::new(),
        }
    }
}