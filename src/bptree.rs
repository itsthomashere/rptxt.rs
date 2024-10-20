//! Implementation of a generic B+ tree
//! A B+ Tree is a self-balanced tree that's only hold value in its leaf nodes.
//! Internal nodes of the tree stores its key and its childs.
//! We can use the keys to index the tree

use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BPTree<T: BPIndex>(pub Rc<TreeNode<T>>);

#[derive(Debug, Clone)]
pub enum TreeNode<T: BPIndex> {
    // An internal node only contains its index and the childs
    // Each child is a subtree
    // TODO: Use something other than a [Vec]
    Internal {
        height: u8,
        index: T::Index,
        childs: Vec<BPTree<T>>,
        childs_index: Vec<T::Index>,
    },
    // Leaf values has its index and values;
    Leaf {
        index: T::Index,
        values: Vec<T>,
        value_index: Vec<T::Index>,
    },
}

pub trait BPIndex: Clone + std::fmt::Debug
where
    Self::Index: Default + Clone + std::fmt::Debug,
{
    type Index;

    fn index(&self) -> Self::Index;
    fn add_index(&mut self, other: Self::Index);
}

impl<T: BPIndex> BPTree<T> {}

impl<T: BPIndex> TreeNode<T> {
    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Leaf { .. })
    }

    pub fn height(&self) -> u8 {
        match self {
            TreeNode::Internal { height, .. } => *height,
            TreeNode::Leaf { .. } => 0,
        }
    }

    pub fn index(&self) -> &T::Index {
        match self {
            TreeNode::Internal { index, .. } => index,
            TreeNode::Leaf { index, .. } => index,
        }
    }

    pub fn child_indexes(&self) -> &[T::Index] {
        match self {
            TreeNode::Internal { childs_index, .. } => childs_index.as_slice(),
            TreeNode::Leaf { value_index, .. } => value_index.as_slice(),
        }
    }

    pub fn child_tree(&self) -> Option<&[BPTree<T>]> {
        match self {
            TreeNode::Internal { childs, .. } => Some(childs.as_slice()),
            TreeNode::Leaf { .. } => None,
        }
    }

    pub fn values(&self) -> Option<&[T]> {
        match self {
            TreeNode::Internal { .. } => None,
            TreeNode::Leaf { values, .. } => Some(values.as_slice()),
        }
    }
}
