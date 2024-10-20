//! Implementation of a generic B+ tree
//! A B+ Tree is a self-balanced tree that's only hold value in its leaf nodes.
//! Internal nodes of the tree stores its key and its childs.
//! We can use the keys to index the tree

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BPTree<T: Clone + std::fmt::Debug>(pub Arc<Node<T>>);

#[derive(Debug, Clone)]
pub enum Node<T>
where
    T: Clone + std::fmt::Debug,
{
    Leaf { values: Vec<T> },
    Internal { height: u8, childs: Vec<BPTree<T>> },
}

impl<T> Node<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf { .. })
    }

    pub fn childs(&self) -> &[BPTree<T>] {
        match self {
            Node::Leaf { .. } => &[],
            Node::Internal { childs, .. } => childs,
        }
    }

    pub fn values(&self) -> &[T] {
        match self {
            Node::Leaf { values } => values,
            Node::Internal { .. } => &[],
        }
    }

    pub fn height(&self) -> u8 {
        match self {
            Node::Leaf { .. } => 0,
            Node::Internal { height, .. } => *height,
        }
    }
}

impl<T> BPTree<T>
where
    T: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self(Arc::new(Node::Leaf { values: Vec::new() }))
    }

    pub fn first(&self) -> Option<&T> {
        Self::walk_left(self)
    }

    pub fn last(&self) -> Option<&T> {
        Self::walk_right(self)
    }

    fn walk_left(node: &Self) -> Option<&T> {
        match node.0.as_ref() {
            Node::Leaf { values } => values.first(),
            Node::Internal { childs, .. } => Self::walk_left(childs.first()?),
        }
    }

    fn walk_right(node: &Self) -> Option<&T> {
        match node.0.as_ref() {
            Node::Leaf { values } => values.last(),
            Node::Internal { childs, .. } => Self::walk_right(childs.last()?),
        }
    }
}

impl<T> Default for BPTree<T>
where
    T: Clone + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_tree() {}
}
