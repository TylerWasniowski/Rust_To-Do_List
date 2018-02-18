use std::clone::Clone;

#[derive(Debug)]
pub struct Database<T> {
    data: Vec<T>,
}

impl <T> Database<T> {
    pub fn add(&mut self, data: T) {
        self.data.push(data);
    }

    pub fn read_all(&self) -> Vec<T> where T: Clone {
        self.data.to_vec()
    }

//    pub fn select_first(&self, predicate: &Fn(T) -> bool) -> Option<T> {
//        self.data.into_iter().find(predicate)
//    }
//
//    pub fn select_all(&self, predicate: &Fn(T) -> bool) -> Option<Vec<T>> {
//        self.data.into_iter().filter(predicate).collect::<Vec<T>>()
//    }
//
//    pub fn remove_first(&self, predicate: &Fn(T) -> bool) {
//        self.data.remove_item(self.select_first(predicate));
//    }
//
//    pub fn remove_all(&self, predicate: &Fn(T) -> bool) {
//        self.data.retain(|&data: T| !predicate(data));
//    }

    pub fn new() -> Database<T> {
        Database {
            data: Vec::new(),
        }
    }
}