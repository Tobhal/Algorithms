use std::borrow::Cow::Borrowed;
use std::collections::VecDeque;
use std::ops;
use std::ptr::null;
use crate::unbalanced::{Counting, Insert};
use crate::unbalanced::array::{Traversal, Util};

pub struct BinaryTree<T>
where T: PartialOrd, T: Copy {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

impl<T> BinaryTree<T>
where T: PartialOrd, T: Copy {
    pub(crate) fn new() -> BinaryTree<T> {
        BinaryTree {
            root: vec![],
            nodes: 0,
            height: 0
        }
    }
    
    pub(crate) fn new_with_data(data: T) -> BinaryTree<T> {
        BinaryTree {
            root: vec![Some(data)],
            nodes: 1,
            height: 1
        }
    }
}

impl<T> Insert<T> for BinaryTree<T>
where T: PartialOrd, T: Copy {
    fn insert(&mut self, data: T) {
        if self.root.len() == 0 {
            self.root.push(Some(data));
            return;
        }

        let mut i = 0;

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

    fn insert_vec(&mut self, data: Vec<T>) {
        for d in data {
            self.insert(d)
        }
    }
}

impl<T> Util<T> for BinaryTree<T>
where T: PartialOrd, T: Copy {
    fn clear(&mut self, idx: usize) {
        if idx > self.root.len() || self.index_out(idx) {return}

        let mut index_queue: VecDeque<usize> = VecDeque::new();

        self.root[idx] = None;

        if self.next_index_out(idx) {return}
        self.add_children_to_queue(idx, &mut index_queue);

        let mut current = idx;

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();
            self.root[current] = None;

            if self.index_out(current) {continue}
            self.add_children_to_queue(current, &mut index_queue);
        }
    }


    fn index_out(&self, idx: usize) -> bool {
        self.left_child(idx) > self.root.len() || self.right_child(idx) > self.root.len() || idx > self.root.len()
    }

    fn next_index_out(&self, idx: usize) -> bool {
        self.left_child(idx) > self.root.len() || self.right_child(idx) > self.root.len()
    }

    fn index_out_f(&self, idx: usize, op: fn(bool, bool) -> bool) -> bool {
        op(
            op(
                self.left_child(idx) > self.root.len(), self.right_child(idx) > self.root.len()),
            usize::from(idx) > self.root.len()
        )
    }

    fn next_index_out_f(&self, idx: usize,  op: fn(bool, bool) -> bool) -> bool {
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

impl<T> Counting for BinaryTree<T>
where T: PartialOrd, T: Copy {
    fn num_nodes(&self) -> u32 {
        if self.index_out(0) {return 0;}
        else if self.next_index_out(0) {return 1;}

        let mut sum: u32 = 1;
        let mut current: usize = 0;

        let mut index_queue: VecDeque<usize> = VecDeque::new();
        self.add_children_to_queue(0, &mut index_queue);

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();
            sum += 1;

            if self.index_out(current) {continue;}
            self.add_children_to_queue(current, &mut index_queue);
        }

        sum
    }

    fn num_leaves(&self) -> u32 {
        if self.index_out(0) {return 1;}

        let mut sum: u32 = 0;
        let mut current: usize = 0;

        let mut index_queue: VecDeque<usize> = VecDeque::new();
        self.add_children_to_queue(0, &mut index_queue);

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();
            if self.next_index_out(current) {
                sum += 1;
                continue;
            }

            if self.root[self.left_child(current)] == None && self.root[self.right_child(current)] == None {
                sum += 1;
                continue;
            }

            self.add_children_to_queue(current, &mut index_queue);
        }

        sum
    }

    fn num_two_children(&self) -> u32 {
        if self.index_out(0) {return 0;}

        let mut sum: u32 = if self.root[self.left_child(0)] != None && self.root[self.right_child(0)] != None {
            1
        } else {
            0
        };
        let mut current: usize = 0;
        let mut index_queue: VecDeque<usize> = VecDeque::new();

        self.add_children_to_queue(0, &mut index_queue);

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();

            if self.index_out(current) {continue;}

            if self.root[self.left_child(current)] != None && self.root[self.right_child(current)] != None {
                sum += 1;
            }

            self.add_children_to_queue(current, &mut index_queue);
        }

        sum
    }

    fn num_levels(&self) -> u32 {
        if self.index_out(0) {return 1;}

        let mut level: u32 = 2;
        let mut current: usize = 0;
        let mut index_queue: VecDeque<usize> = VecDeque::new();
        let mut next_queue: VecDeque<usize> = VecDeque::new();

        self.add_children_to_queue(0, &mut index_queue);

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();

            if self.index_out(current) {continue;}

            if index_queue.is_empty() && !next_queue.is_empty() {
                self.add_children_to_queue(current, &mut index_queue);
                level += 1;

                while !next_queue.is_empty() {
                    index_queue.push_back(next_queue.pop_front().unwrap());
                }
                continue;
            }

            self.add_children_to_queue(current, &mut next_queue);
        }

        level
    }
}

