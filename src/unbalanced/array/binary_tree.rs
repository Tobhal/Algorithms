use std::borrow::Cow::Borrowed;
use std::collections::VecDeque;
use std::ops;
use crate::unbalanced::array::{Counting, Insert, Traversal, Util};

pub struct BinaryTree<T>
where T: PartialOrd, T: Copy {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

impl<T> Insert<T> for BinaryTree<T>
where T: PartialOrd, T: Copy {
    fn insert(&mut self, idx: usize, data: T) {
        if self.root.len() == 0 {
            self.root.push(Some(data));
            return;
        }

        let mut i = idx;

        loop {
            if i >= self.root.len() {
                self.increase_levels(1);
            }

            if self.root[i] == None {
                self.root[i] = Some(data);
                return;
            } else if self.root[i].unwrap() > data {
                i = self.left_child(i)
            } else if self.root[i].unwrap() < data {
                i = self.right_child(i)
            } else if self.root[i].unwrap() == data {
                return;
            }
        }
    }

    fn insert_vec(&mut self, idx: usize, data: Vec<T>) {
        for d in data {
            self.insert(idx, d)
        }
    }
}

impl<T> Util<T> for BinaryTree<T>
where T: PartialOrd, T: Copy {
    fn clear(&mut self, idx: usize) {
        if idx > self.root.len() || self.index_out(idx, ops::BitOr::bitor) {return}

        let mut q: VecDeque<usize> = VecDeque::new();

        self.root[idx] = None;

        if self.next_index_out(idx, ops::BitOr::bitor) {return}
        self.add_children_to_queue(idx, &mut q);

        let mut current = idx;

        while !q.is_empty() {
            current = q.pop_front().unwrap();
            self.root[current] = None;

            if self.index_out(current, ops::BitOr::bitor) {continue}
            self.add_children_to_queue(current, &mut q);
        }
    }

    fn index_out(&self, idx: usize, op: fn(bool, bool) -> bool) -> bool {
        op(
            op(
                self.left_child(idx) > self.root.len(), self.right_child(idx) > self.root.len()),
            usize::from(idx) > self.root.len()
        )
    }

    fn next_index_out(&self, idx: usize,  op: fn(bool, bool) -> bool) -> bool {
        op(self.left_child(idx) > self.root.len(), self.right_child(idx) > self.root.len())
    }

    fn left_child(&self, idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child(&self, idx: usize) -> usize {
        2 * idx + 2
    }

    fn parent(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn add_children_to_queue(&self, idx: usize, q: &mut VecDeque<usize>) {
        if self.root[self.left_child(idx)] != None {
            q.push_back(self.left_child(idx));
        }
        if self.root[self.right_child(idx)] != None {
            q.push_back(self.right_child(idx));
        }
    }

    fn increase_levels(&mut self, amount: u32) {
        self.height += 1;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None)
    }
}

