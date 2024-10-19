//! A B+ Tree is a self-balanced tree
//! The tree maintained order, and only the leaf node contains values

use std::rc::Rc;

pub struct BPTree<T: Summarizable>(pub Rc<BPNode<T>>);

pub enum BPNode<T: Summarizable> {
    Internal {
        height: u8,
        summary: T::Summary,
        childs: Vec<BPTree<T>>,
        child_summaries: Vec<T::Summary>,
    },
    Leaf {
        summary: T::Summary,
        items: Vec<T>,
        item_summaries: Vec<T>,
    },
}

/// Summarizable is the required traits for BPTree items
/// See [Summary]
pub trait Summarizable: Clone + std::fmt::Debug {
    type Summary: Summary;

    fn summary(&self);
}

/// Summary represent the unique index of a type
/// Summary of the same type needs to be able to add together
pub trait Summary: Clone + std::fmt::Debug {
    fn add(&mut self, other: Self);
}
