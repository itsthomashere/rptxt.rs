use std::slice::Iter;
use std::sync::Arc;

const MAX_LEN: u8 = 8;
#[derive(Clone)]
#[repr(transparent)]
pub struct BinaryTree<T>(Arc<BinaryNode<T>>);

#[derive(Clone)]
pub enum BinaryNode<T> {
    Internal {
        sum_len: u8,
        left: Option<Arc<BinaryNode<T>>>,
        right: Option<Arc<BinaryNode<T>>>,
    },
    Leaf {
        value: T,
        len: u8,
    },
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self(Arc::new(BinaryNode::Internal {
            sum_len: 0,
            left: None,
            right: None,
        }))
    }

    pub fn insert(&mut self, index: usize) {
        unimplemented!()
    }

    pub fn delete(&mut self, index: usize) {}

    pub fn is_balanced(&self) -> bool {
        unimplemented!()
    }

    pub fn iter(&self) -> Iter<BinaryNode<T>> {
        unimplemented!()
    }

    pub fn iter_mut(&mut self) -> Iter<BinaryNode<T>> {
        unimplemented!()
    }

    pub fn inorder(&self) -> Vec<T> {
        unimplemented!()
    }
}

impl<T> BinaryTree<T>
where
    T: Eq + PartialEq,
{
    pub fn bfs(&self, target: T) -> Option<usize> {
        unimplemented!()
    }

    pub fn dfs(&self, target: T) -> Option<usize> {
        unimplemented!()
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
