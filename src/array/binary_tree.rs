use std::borrow::Cow::Borrowed;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops;
use std::ptr::null;
use std::str::ParseBoolError;
use crate::Node;
use crate::utils::util::{
    Counting, Utility, OrderedTraversal, BFS, Insert, InsertAt, Util, Contains, Remove
};

pub struct BinaryTree<T> {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

/*
crate::impl_utils!(BinaryTree<T: PartialOrd + Copy + Display>);
crate::impl_ordered_traversal!(BinaryTree<T: PartialOrd + Copy + Display>);
crate::impl_BFS!(BinaryTree<T: PartialOrd + Copy + Display>);
crate::impl_counting!(BinaryTree<T: PartialOrd + Copy + Display>);
 */

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

impl<T> Counting for BinaryTree<T>
where T: PartialOrd + Copy {
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
        if self.next_index_out(0) {return 1;}
        if self.index_out(0) {return 0};

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

            if self.root[BinaryTree::<T>::left_child(current)] == None && self.root[BinaryTree::<T>::right_child(current)] == None {
                sum += 1;
                continue;
            }

            self.add_children_to_queue(current, &mut index_queue);
        }

        sum
    }

    fn num_two_children(&self) -> u32 {
        if self.index_out(0) {return 0;}

        let mut sum: u32 = if self.root[BinaryTree::<T>::left_child(0)] != None && self.root[BinaryTree::<T>::right_child(0)] != None {
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

            if self.root[BinaryTree::<T>::left_child(current)] != None && self.root[BinaryTree::<T>::right_child(current)] != None {
                sum += 1;
            }

            self.add_children_to_queue(current, &mut index_queue);
        }

        sum
    }

    fn num_levels(&self) -> u32 {
        if self.next_index_out(0) {return 1;}
        if self.index_out(0) {return 0;}

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

impl<T> Utility for BinaryTree<T>
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

impl<T> OrderedTraversal<T> for BinaryTree<T>
where T: PartialOrd + Copy {
    fn pre_order(&self) -> Vec<T> {
        self.pre_order_from(0)
    }
    fn pre_order_from(&self, idx: usize) -> Vec<T> {
        if self.root.is_empty() {return vec![];}

        if self.next_index_out(idx) && self.root[idx] != None {
            vec![self.root[idx].unwrap()];
        }

        let mut return_vec: Vec<T> = Vec::new();
        let mut next_index: Vec<usize> = vec![idx];

        let mut i: usize = idx;

        while !next_index.is_empty() {
            i = next_index.pop().unwrap();

            if self.root[i] != None {
                return_vec.push(self.root[i].unwrap())
            }

            if self.next_index_out(i) {continue;}

            if self.root[BinaryTree::<T>::right_child(i)] != None {
                next_index.push(BinaryTree::<T>::right_child(i));
            }

            if self.root[BinaryTree::<T>::left_child(i)] != None {
                next_index.push(BinaryTree::<T>::left_child(i));
            }
        }

        return_vec
    }

    fn in_order(&self) -> Vec<T> {
        self.in_order_from(0)
    }
    fn in_order_from(&self, idx: usize) -> Vec<T> {
        if self.root.is_empty() || self.root[idx] == None {return vec![];}

        if self.next_index_out(idx) && self.root[idx] != None {
            return vec![self.root[idx].unwrap()];
        }

        let mut return_vec: Vec<T> = Vec::new();
        let mut next_index: Vec<usize> = vec![idx];
        let mut done_idx: Vec<bool> = vec![false; self.nodes as usize];

        let mut i: usize = idx;
        let num_nodes = self.num_nodes() as usize;

        while !next_index.is_empty() {
            i = next_index.pop().unwrap();

            if self.root[i] == None {
                continue;
            }

            // Is next nodes outside the vec, add parent node to stack
            if self.next_index_out(i) {
                done_idx[i] = true;
                return_vec.push(self.root[i].unwrap());
                i = BinaryTree::<T>::parent(i);
            }

            // Add left child to stack so the value can be added later
            if self.root[BinaryTree::<T>::left_child(i)] != None && !done_idx[BinaryTree::<T>::left_child(i)] {
                next_index.push(BinaryTree::<T>::left_child(i));
                continue;
            }

            // If node not visited before, add the value to output vector
            if !done_idx[i] {
                return_vec.push(self.root[i].unwrap());
                done_idx[i] = true;
            }

            // Add right child to stack so the value can be added later
            if self.root[BinaryTree::<T>::right_child(i)] != None && !done_idx[BinaryTree::<T>::right_child(i)] {
                next_index.push(BinaryTree::<T>::right_child(i));
                continue;
            }

            // Add the parent to the stack to move up the tree
            if i > idx {
                next_index.push(BinaryTree::<T>::parent(i));
            }

            // If all nodes are visited return the vector
            if num_nodes <= return_vec.len() {return return_vec;}
        }

        return_vec
    }

    fn post_order(&self) -> Vec<T> {
        self.post_order_from(0)
    }
    fn post_order_from(&self, idx: usize) -> Vec<T> {
        if self.root.is_empty() || self.root[0] == None {return vec![];}

        if self.next_index_out(idx) && self.root[idx] != None {
            return vec![self.root[idx].unwrap()];
        }

        let mut return_vec: Vec<T> = Vec::new();
        let mut next_index: Vec<usize> = vec![idx];
        let mut done_idx: Vec<bool> = vec![false; self.nodes as usize];

        let mut i: usize = idx;
        let num_nodes = self.num_nodes() as usize;

        while !next_index.is_empty() {
            i = next_index.pop().unwrap();

            if self.root[i] == None {
                continue;
            }

            // Is next nodes outside the vec, add parent node to stack
            if self.next_index_out(i) {
                done_idx[i] = true;
                return_vec.push(self.root[i].unwrap());
                i = BinaryTree::<T>::parent(i);
            }

            // Add left child to stack so the value can be added later
            if self.root[BinaryTree::<T>::left_child(i)] != None && !done_idx[BinaryTree::<T>::left_child(i)] {
                next_index.push(BinaryTree::<T>::left_child(i));
                continue;
            }

            // Add right child to stack so the value can be added later
            if self.root[BinaryTree::<T>::right_child(i)] != None && !done_idx[BinaryTree::<T>::right_child(i)] {
                next_index.push(BinaryTree::<T>::right_child(i));
                continue;
            }

            // If node not visited before, add the value to output vector
            if !done_idx[i] {
                return_vec.push(self.root[i].unwrap());
                done_idx[i] = true;
            }

            // Add the parent to the stack to move up the tree
            if i > idx {
                next_index.push(BinaryTree::<T>::parent(i));
            }

            // If all nodes are visited return the vector
            if num_nodes <= return_vec.len() {
                return return_vec;
            }
        }

        return_vec
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

impl<T> Contains<T> for BinaryTree<T>
where T: PartialOrd + Copy {
    fn contains(&self, data: T) -> bool {
        let mut i: usize = 0;

        loop {
            if self.root[i] != None && self.root[i].unwrap() == data {return true;}

            if self.next_index_out(i) || self.root[i] == None {return false;}

            i = self.get_child(i, data).unwrap();
        }
    }

    fn find(&self, data: T) -> Result<usize, String> {
        let mut tmp = 0;

        while self.root[tmp] != None && self.root[tmp].unwrap() != data {
            tmp = match self.get_child(tmp, data) {
                Err(e) => {return Err(e)},
                Ok(val) => {
                    val
                }
            }
        }

        Ok(tmp)
    }
}

impl<T> Remove<T> for BinaryTree<T>
where T: PartialOrd + Copy {
    fn remove(&mut self, data: T) {
        if self.root.len() == 0 {return;}

        // Search for element, if not found just retorn
        let mut tmp = match self.find(data) {
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

impl<T> BFS<T> for BinaryTree<T>
where T: PartialOrd + Copy {
    fn bfs(&self) -> Vec<T> {
        self.bfs_from(0)
    }
    fn bfs_from(&self, idx: usize) -> Vec<T> {
        if self.root.is_empty() || self.root[idx] == None {return vec![];}

        if self.next_index_out(idx) {return vec![self.root[idx].unwrap()];}

        let mut return_vec: Vec<T> = vec![self.root[idx].unwrap()];
        let mut index_queue: VecDeque<usize> = VecDeque::new();

        self.add_children_to_queue(idx, &mut index_queue);

        let mut current: usize = idx;

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();

            return_vec.push(self.root[current].unwrap());

            if self.index_out(current) {continue;}

            self.add_children_to_queue(current, &mut index_queue);
        }

        return_vec
    }
}