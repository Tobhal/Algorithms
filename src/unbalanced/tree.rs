use crate::utils::util::Node;

pub mod binary_tree;

pub(crate) trait Insert<T> {
    fn insert(&self, data: T);
}

pub(crate) trait Remove<T> {
    fn remove(data: T);
}

pub(crate) trait Contains<T> {
    fn contains(data: T);
}

pub(crate) trait Counting {
    fn num_nodes() -> i8;
    fn num_leaves() -> i8;
    fn num_two_children() -> i8;
    fn num_levels() -> i8;
}

pub(crate) trait Traversal<T> {
    fn pre_order() -> Vec<Node<T>>;
    fn in_order() -> Vec<Node<T>>;
    fn post_order() -> Vec<Node<T>>;
    fn bfs() -> Vec<Node<T>>;
}