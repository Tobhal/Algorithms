use crate::unbalanced::tree::{Counting, Insert, Traversal};
use super::super::super::utils::util::Node;

pub struct BTree<T> {
    pub(crate) root: Option<Node<T>>
}

impl<T> BTree<T> {
    fn insert(data: T, root: Option<Node<T>>) -> Node<T> {
        if root.is_none() {
            return Node {data, ..Default::default()};
        }

        let mut node = root.unwrap();

        if data < node.data {
            node.left = insert(data, node.left);
        } else if data > node.data {
            node.right = insert(data, node.right);
        } else {
            node.count += 1;
        }

        return node;
    }
}

impl<T> Insert<T> for BTree<T> {
    fn insert(&mut self, data: T) {
        // Root is None
        if self.root.is_none() {
            self.root = Option::from(Node { data, ..Default::default() });
            return;
        }

        // Root is not none
        let mut node = &self.root;
        while node.is_some() {
            let node_data = node.unwrap().data;
            if node_data < data {
                node = &node.unwrap().left;
                continue
            } else if node_data > data {
                node = &node.unwrap().right;
                continue
            } else {
                node.unwrap().count += 1;
            }
        }

    }
}

impl<T> Counting for BTree<T> {
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

impl<T> Traversal<T> for BTree<T> {
    fn pre_order() -> Vec<Node<T>> {
        todo!()
    }

    fn in_order() -> Vec<Node<T>> {
        todo!()
    }

    fn post_order() -> Vec<Node<T>> {
        todo!()
    }

    fn bfs() -> Vec<Node<T>> {
        todo!()
    }
}