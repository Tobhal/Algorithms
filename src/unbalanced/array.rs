use std::collections::VecDeque;

pub mod binary_tree;

pub(crate) trait Util<T> {
    fn clear_from(&mut self, idx: usize);

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

pub(crate) trait InsertAt<T>
{
    fn insert_at(&mut self, idx: usize, data: T);
    fn insert_vec_at(&mut self, idx: usize, data: Vec<T>);
}

pub(crate) trait Clear<T> {
    fn clear_at(&mut self, idx: usize);
}

pub(crate) trait TraversalFrom<T> {
    fn pre_order_from(&self, idx: usize) -> Vec<T>;
    fn in_order_from(&self, idx: usize) -> Vec<T>;
    fn post_order_from(&self, idx: usize) -> Vec<T>;
    fn bfs_from(&self, idx: usize) -> Vec<T>;
}