use std::collections::VecDeque;

pub mod binary_tree;

pub(crate) trait Insert<T> {
    fn insert(&mut self, idx: usize, data: T);
    fn insert_vec(&mut self, idx: usize, data: Vec<T>);
}

pub(crate) trait Remove<T> {
    fn remove(&mut self, data: T);
}

pub(crate) trait Contains<T> {
    fn contains(&self, data: T) -> bool;
}

pub(crate) trait Counting {
    fn num_nodes(&self) -> i8;
    fn num_leaves(&self) -> i8;
    fn num_two_children(&self) -> i8;
    fn num_levels(&self) -> i8;
}

pub(crate) trait Traversal<T> {
    fn pre_order(&self) -> Vec<T>;
    fn in_order(&self) -> Vec<T>;
    fn post_order(&self) -> Vec<T>;
    fn bfs(&self) -> Vec<T>;
}

pub(crate) trait Print<T> {
    fn print(&self, f: fn () -> Vec<T>);
    fn println(&self, f: fn () -> Vec<T>);
}

pub(crate) trait Util<T> {
    fn clear(&mut self, idx: usize);

    fn index_out(&self, idx: usize, op: fn (bool, bool) -> bool) -> bool;
    fn next_index_out(&self, idx: usize, op: fn (bool, bool) -> bool) -> bool;

    fn left_child(&self, idx: usize) -> usize;
    fn right_child(&self, idx: usize) -> usize;
    fn parent(&self, idx: usize) -> usize;

    fn add_children_to_queue(&self, idx: usize, q: &mut VecDeque<usize>);

    fn increase_levels(&mut self, amount: u32);
}