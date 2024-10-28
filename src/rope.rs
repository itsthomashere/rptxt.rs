use crate::bintree::BinaryTree;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Clone)]
pub struct Rope<T> {
    inner: BinaryTree<T>,
}

impl<T> Rope<T> {
    pub fn append(&mut self, other: Self) {
        unimplemented!()
    }

    pub fn insert(&mut self, index: usize, value: T) {
        unimplemented!()
    }

    pub fn split(&mut self, index: usize) -> (&Self, &Self) {
        unimplemented!()
    }

    pub fn is_balanced(&self) -> bool {
        unimplemented!()
    }

    pub fn delete_span(&mut self, start: usize, end: usize) {
        unimplemented!()
    }

    pub fn iter(&self) -> Iter<T> {
        unimplemented!()
    }

    pub fn collect(&self) -> Vec<T> {
        unimplemented!()
    }
}

impl<String> FromStr for Rope<String> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl Rope<String> {
    pub fn string(&self) -> String {
        let mut result = String::new();

        self.collect().iter().for_each(|st| result += st);
        result
    }
}
