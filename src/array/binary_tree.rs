use std::borrow::Cow::Borrowed;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops;
use std::ptr::null;
use std::str::ParseBoolError;
use crate::{impl_BFS, impl_contains, impl_counting, impl_ordered_traversal, impl_utils};
use crate::utils::util::{
    Counting, Utility, OrderedTraversal, BFS, Insert, InsertAt, Util, Contains, Remove, Direction
};

pub struct BinaryTree<T> {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

impl_utils!(BinaryTree<T: PartialOrd>);
impl_counting!(BinaryTree<T: PartialOrd>);
impl_ordered_traversal!(BinaryTree<T: PartialOrd + Copy>);
impl_BFS!(BinaryTree<T: PartialOrd + Copy>);
impl_contains!(BinaryTree<T: PartialOrd + Copy>);

impl<T> BinaryTree<T> {
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
where T: PartialOrd + Copy {
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
            } else if self.root[i].unwrap() == data {
                return;
            }

            i = self.get_child(i, data).unwrap();
        }
    }

    fn insert_vec_at(&mut self, idx: usize, data: Vec<T>) {
        for d in data {
            self.insert_at(idx, d);
        }
    }
}

impl<T> Insert<T> for BinaryTree<T>
where T: PartialOrd + Copy {
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

impl<T> Remove<T> for BinaryTree<T>
where T: PartialOrd + Copy {
    fn remove(&mut self, data: T) {
        if self.root.len() == 0 {return;}

        // Search for element, if not found just retorn
        let tmp = match self.find(data) {
            Ok(val) => val,
            Err(_) => {return;}
        };

        if self.root[tmp] == None {return;}

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

impl<T> Util<T> for BinaryTree<T>
where T: PartialOrd + Copy {
    fn clear_from(self: &mut BinaryTree<T>, idx: usize) {
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

    fn decrease_levels(&mut self, amount: u32) {
        self.height -= 1;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None);
    }

    fn get_child(&self, idx: usize, data: T) -> Result<usize, String> {
         match self.root[idx] {
            None => { Err("No node found".to_string()) }
            Some(val) => {
                if val > data {
                    Ok(BinaryTree::<T>::left_child(idx))
                } else {
                    Ok(BinaryTree::<T>::right_child(idx))
                }
            }
        }
    }
}
