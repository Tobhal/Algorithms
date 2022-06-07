use std::collections::VecDeque;

pub mod binary_tree;

pub(crate) trait Traversal<T> {
    fn pre_order(&self) -> Vec<T>;
    fn in_order(&self) -> Vec<T>;
    fn post_order(&self) -> Vec<T>;
    fn bfs(&self) -> Vec<T>;
}

pub(crate) trait Util<T> {
    fn clear(&mut self, idx: usize);

    fn index_out(&self, idx: usize) -> bool;
    fn next_index_out(&self, idx: usize) -> bool;

    fn index_out_f(&self, idx: usize, op: fn (bool, bool) -> bool) -> bool;
    fn next_index_out_f(&self, idx: usize, op: fn (bool, bool) -> bool) -> bool;

    fn left_child(&self, idx: usize) -> usize;
    fn right_child(&self, idx: usize) -> usize;
    fn parent(&self, idx: usize) -> usize;

    fn add_children_to_queue(&self, idx: usize, q: &mut VecDeque<usize>);

    fn increase_levels(&mut self, amount: u32);
}