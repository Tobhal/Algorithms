use std::borrow::Cow::Borrowed;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops;
use std::ptr::null;
use std::str::ParseBoolError;
use crate::unbalanced::{Insert, Contains, Remove};
use crate::unbalanced::array::{InsertAt, Util};
use crate::utils::util::{Utility, OrderedTraversal, BFS, Counting};

pub struct BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

crate::impl_utils!(BinaryTree<T: PartialOrd + Copy + Display>);
crate::impl_ordered_traversal!(BinaryTree<T: PartialOrd + Copy + Display>);
crate::impl_BFS!(BinaryTree<T: PartialOrd + Copy + Display>);
crate::impl_counting!(BinaryTree<T: PartialOrd + Copy + Display>);

impl<T> BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
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

impl<T> InsertAt<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn insert_at(&mut self, idx: usize, data: T) {
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
                i = BinaryTree::<T>::left_child(i)
            } else if self.root[i].unwrap() < data {
                i = BinaryTree::<T>::right_child(i)
            } else if self.root[i].unwrap() == data {
                return;
            }
        }
    }

    fn insert_vec_at(&mut self, idx: usize, data: Vec<T>) {
        for d in data {
            self.insert_at(idx, d);
        }
    }
}

impl<T> Insert<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn insert(&mut self, data: T) {
        if self.root.len() == 0 {
            self.root.push(Some(data));
            return;
        }

        self.insert_at(0, data);
    }

    fn insert_vec(&mut self, data: Vec<T>) {
        self.insert_vec_at(0, data)
    }
}

impl<T> Contains<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn contains(&self, data: T) -> bool {
        let mut i: usize = 0;

        loop {
            if self.root[i] != None && self.root[i].unwrap() == data {return true;}

            if self.next_index_out(i) || self.root[i] == None {return false;}

            i = if self.root[i].unwrap() > data {
                BinaryTree::<T>::left_child(i)
            } else {
                BinaryTree::<T>::right_child(i)
            };
        }
    }
}

impl<T> Remove<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn remove(&mut self, data: T) {
        if self.root.len() == 0 {return;}

        // Temp index
        let mut tmp: usize = 0;

        // Search for element
        // TODO: Write find function?
        while self.root[tmp] != None && self.root[tmp].unwrap() != data {
            tmp = if self.root[tmp].unwrap() > data {
                BinaryTree::<T>::left_child(tmp)
            } else {
                BinaryTree::<T>::right_child(tmp)
            };
        }

        if self.root[tmp] != None {
            // Case 1 - Delete leaf Node
            if self.next_index_out(tmp) {
                self.root[tmp] = None;
                return;
            } else {
                if self.root[BinaryTree::<T>::left_child(tmp)] == None && self.root[BinaryTree::<T>::right_child(tmp)] == None {
                    self.root[tmp] = None;
                    return;
                }
                // Case 2 - Delete node with one child
                else if self.root[BinaryTree::<T>::left_child(tmp)] == None || self.root[BinaryTree::<T>::right_child(tmp)] == None {
                    if self.root[BinaryTree::<T>::left_child(tmp)] == None {
                        let el = self.bfs_from(BinaryTree::<T>::right_child(tmp));
                        self.clear_from(tmp);
                        self.insert_vec_at(tmp, el);
                    } else {
                        let el = self.bfs_from(BinaryTree::<T>::left_child(tmp));
                        self.clear_from(tmp);
                        self.insert_vec_at(tmp, el);
                    }
                }
                // Case 3 - Delete Node with 2 children
                else {
                    let mut child = BinaryTree::<T>::left_child(tmp);
                    loop {
                        if BinaryTree::<T>::right_child(child) > self.root.len() {break;}

                        if self.root[BinaryTree::<T>::right_child(child)] == None {break;}

                        child = BinaryTree::<T>::right_child(child);
                    }
                    let new_childs = self.bfs_from(BinaryTree::<T>::left_child(child));
                    self.clear_from(BinaryTree::<T>::left_child(child));
                    self.root[tmp] = self.root[child];
                    self.root[child] = None;
                    self.insert_vec_at(child, new_childs);
                }
            }
        }
    }
}

impl<T> Util<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn clear_from(&mut self, idx: usize) {
        if idx > self.root.len() || self.index_out(idx) || self.next_index_out(idx) {return}

        let mut index_queue: VecDeque<usize> = VecDeque::new();

        self.root[idx] = None;

        self.add_children_to_queue(idx, &mut index_queue);

        let mut current = idx;

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();
            self.root[current] = None;

            if self.index_out(current) {continue}
            self.add_children_to_queue(current, &mut index_queue);
        }
    }

    fn increase_levels(&mut self, amount: u32) {
        self.height += 1;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None)
    }
}
