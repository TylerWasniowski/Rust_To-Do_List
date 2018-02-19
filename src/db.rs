use std::ops::Fn;
use std::ops::FnMut;

#[derive(Debug)]
pub struct Database<T> {
    data: Vec<T>,
}

impl<T> Database<T> {
    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn read_all(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
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

    pub fn replace(&mut self, index: usize, new_value: T) {
        if let Some(value) = self.data.get_mut(index) {
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

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.get(index) {
            Some(_) => Some(self.data.remove(index)),
            None => None,
        }
    }

    pub fn remove_first<F: Fn(&T) -> bool>(&mut self, predicate: F) -> Option<T> {
        match self.data.iter()
            .position(predicate)
        {
            Some(index) => Some(self.data.remove(index)),
            None => None,
        }
    }

    pub fn remove_all<F: Fn(&T) -> bool>(&mut self, predicate: F) {
        self.data.retain(|value| !predicate(value));
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn new() -> Database<T> {
        Database {
            data: Vec::new(),
        }
    }
}