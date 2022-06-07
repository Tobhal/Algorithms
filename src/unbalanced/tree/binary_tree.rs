use crate::unbalanced::tree::{Counting, Insert, Traversal};
use super::super::super::utils::util::Node;

pub struct BinaryTree<T> {
    pub(crate) root: Option<Node<T>>
}

impl<T> BinaryTree<T> {
    fn insert(data: T, root: Option<Node<T>>) -> Node<T> {
        todo!()
    }
}

impl<T> Insert<T> for BinaryTree<T> {
    fn insert(&self, data: T) {
        todo!()
    }
}

impl<T> Counting for BinaryTree<T> {
    fn num_nodes() -> i8 {
        todo!()
    }

    fn num_leaves() -> i8 {
        todo!()
    }

    fn num_two_children() -> i8 {
        todo!()
    }

    fn num_levels() -> i8 {
        todo!()
    }
}