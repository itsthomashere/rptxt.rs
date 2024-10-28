//! Implementation of a generic B+ tree
//! A B+ Tree is a self-balanced tree that's only hold value in its leaf nodes.
//! Internal nodes of the tree stores its key and its childs.
//! We can use the keys to index the tree

use std::sync::Arc;

const TREE_BASE: usize = 6;

#[derive(Clone)]
pub struct BPTree<T: TreeIdx>(Arc<Node<T>>);

#[derive(Clone)]
pub enum Node<T: TreeIdx> {
    Internal {
        height: u8,
        childs: Vec<BPTree<T>>,
        index: T::Index,
        child_indexes: Vec<T::Index>,
    },
    Leaf {
        index: T::Index,
        item_indexes: Vec<T::Index>,
        items: Vec<T>,
    },
}

impl<T: TreeIdx> Node<T> {
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf { .. })
    }

    pub fn height(&self) -> u8 {
        match self {
            Node::Internal { height, .. } => *height,
            Node::Leaf { .. } => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Node::Internal { childs, .. } => childs.is_empty(),
            Node::Leaf { items, .. } => items.is_empty(),
        }
    }

    pub fn child_indexes(&self) -> &[T::Index] {
        match self {
            Node::Internal { child_indexes, .. } => child_indexes.as_slice(),
            Node::Leaf { item_indexes, .. } => item_indexes.as_slice(),
        }
    }

    pub fn child_tree(&self) -> &[BPTree<T>] {
        match self {
            Node::Internal { childs, .. } => childs.as_slice(),
            Node::Leaf { .. } => Default::default(),
        }
    }

    pub fn items(&self) -> &[T] {
        match self {
            Node::Internal { .. } => Default::default(),
            Node::Leaf { items, .. } => items.as_slice(),
        }
    }

    pub fn is_underflowing(&self) -> bool {
        match self {
            Node::Internal { childs, .. } => childs.len() < TREE_BASE,
            Node::Leaf { items, .. } => items.len() < TREE_BASE,
        }
    }
}

impl<T: TreeIdx> BPTree<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self(Arc::new(Node::Leaf {
            index: Default::default(),
            item_indexes: Vec::default(),
            items: Vec::default(),
        }))
    }
}

impl<T: TreeIdx> Default for BPTree<T>
where
    T: Clone + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait TreeIdx: Clone + Default {
    type Index: Summary;

    fn index(&self) -> Self::Index;
}

pub trait Summary: Clone + Default {
    fn add_summary(&mut self, other: Self);
}
