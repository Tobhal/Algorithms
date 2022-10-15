use std::collections::VecDeque;
use std::fmt::Display;
use crate::array::binary_tree::BinaryTree;
use crate::utils::util::{Utility, Util, Insert, InsertAt};


pub struct AVLTree<T> {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) balanceFactor: Vec<u32>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

/*
crate::impl_utils!(AVLTree<T: PartialOrd + Copy + Display>);
crate::impl_ordered_traversal!(AVLTree<T: PartialOrd + Copy + Display>);
crate::impl_BFS!(AVLTree<T: PartialOrd + Copy + Display>);
// crate::impl_counting!(AVLTree<T: PartialOrd + Copy + Display>);
*/

impl<T> AVLTree<T> {
    pub(crate) fn new() -> AVLTree<T> {
        AVLTree {
            root: vec![],
            balanceFactor: vec![],
            nodes: 0,
            height: 0
        }
    }
}

impl<T> Utility for AVLTree<T>
where T: PartialOrd + Copy {
    fn index_out(&self, idx: usize) -> bool {
        BinaryTree::<T>::left_child(idx) > self.root.len() || BinaryTree::<T>::right_child(idx) > self.root.len() || idx > self.root.len()
    }

    fn next_index_out(&self, idx: usize) -> bool {
        BinaryTree::<T>::left_child(idx) > self.root.len() || BinaryTree::<T>::right_child(idx) > self.root.len()
    }

    fn index_out_f(&self, idx: usize, op: fn(bool, bool) -> bool) -> bool {
        op(
            op(
                BinaryTree::<T>::left_child(idx) > self.root.len(), BinaryTree::<T>::right_child(idx) > self.root.len()),
            usize::from(idx) > self.root.len()
        )
    }

    fn next_index_out_f(&self, idx: usize,  op: fn(bool, bool) -> bool) -> bool {
        op(BinaryTree::<T>::left_child(idx) > self.root.len(), BinaryTree::<T>::right_child(idx) > self.root.len())
    }

    fn left_child(idx: usize) -> usize {2 * idx  + 1}
    fn right_child(idx: usize) -> usize {2 * idx  + 2}
    fn parent(idx: usize) -> usize {(idx - 1) / 2}

    fn add_children_to_queue(&self, idx: usize, q: &mut VecDeque<usize>) {
        if self.root[BinaryTree::<T>::left_child(idx)] != None {
            q.push_back(BinaryTree::<T>::left_child(idx));
        }
        if self.root[BinaryTree::<T>::right_child(idx)] != None {
            q.push_back(BinaryTree::<T>::right_child(idx));
        }
    }
}

pub(crate) trait Rotate {
    fn rotate_left(&mut self, idx: usize);

    fn rotate_right(&mut self, idx: usize);

    fn rotate_left_right(&mut self, idx: usize);
    
    fn rotate_right_left(&mut self, idx: usize);
}

impl<T> Rotate for AVLTree<T>
where T: PartialOrd + Copy {
    fn rotate_left(&mut self, idx: usize) {
        let parentIdx = AVLTree::<T>::parent(idx);
        let parentVal = self.root[parentIdx];

        let mut currentNode = idx;

        loop {
            self.root[AVLTree::<T>::parent(currentNode)] = self.root[currentNode];
            self.root[currentNode] = None;

            if self.index_out(currentNode) {
                break;
            }

            currentNode = AVLTree::<T>::right_child(currentNode);
        }

        self.root[AVLTree::<T>::left_child(parentIdx)] = parentVal;

    }

    fn rotate_right(&mut self, idx: usize) {
        let parentIdx = AVLTree::<T>::parent(idx);
        let parentVal = self.root[parentIdx];

        let mut currentNode = idx;

        loop {
            self.root[AVLTree::<T>::parent(currentNode)] = self.root[currentNode];
            self.root[currentNode] = None;

            if self.index_out(currentNode) {
                break;
            }

            currentNode = AVLTree::<T>::left_child(currentNode);
        }

        self.root[AVLTree::<T>::right_child(parentIdx)] = parentVal;
    }

    fn rotate_left_right(&mut self, idx: usize) {
        todo!()
    }

    fn rotate_right_left(&mut self, idx: usize) {
        todo!()
    }
}







/*
impl<T> InsertAt<T> for AVLTree<T>
where T: PartialOrd + Copy + Display {
    fn insert_at(&mut self, idx: usize, data: T) {
        /*
        Do normal insert
         */
        if self.root.len() == 0 {
            self.root.push(Some(data));
            self.balanceFactor[0] = 0;
            return;
        }

        let mut i = idx;

        loop {
            if i >= self.root.len() {
                self.increase_levels(1);
            }

            if self.root[i] == None && self.balanceFactor[i] == 0 {
                self.root[i] = Some(data);
                break;
            } else if self.root[i].unwrap() > data {
                i = BinaryTree::<T>::left_child(i)
            } else if self.root[i].unwrap() < data {
                i = BinaryTree::<T>::right_child(i)
            } else if self.root[i].unwrap() == data {
                break;
            }
        }

        /*
        Update balance factor

        fix later
         */
        self.balanceFactor[i] = 0;
        let mut _prev_data = &data;
        /*
        loop {
            i = BinaryTree::<T>::parent(i);

            // update parent node balance factor
            if self.root[i].unwrap() > prev_data {
                self.balanceFactor -= 1;
            } else {
                self.balanceFactor += 1;
            }

            if self.balanceFactor[i] > 2 {
                // Rotate
                if self.root[i].unwrap() > prev_data {
                    self.rotate_left(AVLTree::<T>::left_child(i));
                } else {
                    self.root_right(AVLTree::<T>::right_child(i));
                }

                i = AVLTree::<T>::parent(i);
            }
        }
         */

    }

    fn insert_vec_at(&mut self, idx: usize, data: Vec<T>) {
        todo!()
    }
}

impl<T> Insert<T> for AVLTree<T>
where T: PartialOrd + Copy + Display {
    fn insert(&mut self, data: T) {
        self.insert_at(0, data)
    }

    fn insert_vec(&mut self, data: Vec<T>) {
        self.insert_vec_at(0, data)
    }
}

impl<T> Util<T> for AVLTree<T>
where T: PartialOrd + Copy + Display {
    fn clear_from(&mut self, idx: usize) {
        todo!()
    }

    fn increase_levels(&mut self, amount: u32) {
        self.height += 1;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None);
        self.balanceFactor.resize_with(self.nodes as usize, || 0);
    }

    fn get_child(&self, idx: usize, data: T) -> Result<usize, String> {
        todo!()
    }
}
 */