use std::borrow::Cow::Borrowed;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops;
use std::ptr::null;
use std::str::ParseBoolError;
use crate::unbalanced::{Counting, Insert, Traversal, Contains, Remove};
use crate::unbalanced::array::{InsertAt, TraversalFrom, Util};

pub struct BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

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
                i = self.left_child(i)
            } else if self.root[i].unwrap() < data {
                i = self.right_child(i)
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
                self.left_child(i)
            } else {
                self.right_child(i)
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
                self.left_child(tmp)
            } else {
                self.right_child(tmp)
            };
        }

        if self.root[tmp] != None {
            // Case 1 - Delete leaf Node
            if self.next_index_out(tmp) {
                self.root[tmp] = None;
                return;
            } else {
                if self.root[self.left_child(tmp)] == None && self.root[self.right_child(tmp)] == None {
                    self.root[tmp] = None;
                    return;
                }
                // Case 2 - Delete node with one child
                else if self.root[self.left_child(tmp)] == None || self.root[self.right_child(tmp)] == None {
                    if self.root[self.left_child(tmp)] == None {
                        let el = self.bfs_from(self.right_child(tmp));
                        self.clear_from(tmp);
                        self.insert_vec_at(tmp, el);
                    } else {
                        let el = self.bfs_from(self.left_child(tmp));
                        self.clear_from(tmp);
                        self.insert_vec_at(tmp, el);
                    }
                }
                // Case 3 - Delete Node with 2 children
                else {
                    let mut child = self.left_child(tmp);
                    loop {
                        if self.right_child(child) > self.root.len() {break;}

                        if self.root[self.right_child(child)] == None {break;}

                        child = self.right_child(child);
                    }
                    let new_childs = self.bfs_from(self.left_child(child));
                    self.clear_from(self.left_child(child));
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
where T: PartialOrd, T: Copy, T: Display {
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

impl<T> Traversal<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn pre_order(&self) -> Vec<T> {
        if self.root.is_empty() {return vec![];}

        if self.next_index_out(0) && self.root[0] != None {
            vec![self.root[0].unwrap()];
        }

        let mut return_vec: Vec<T> = Vec::new();
        let mut next_index: Vec<usize> = vec![0];

        let mut i: usize;

        while !next_index.is_empty() {
            i = next_index.pop().unwrap();

            if self.root[i] != None {
                return_vec.push(self.root[i].unwrap())
            }

            if self.next_index_out(i) {continue;}

            if self.root[self.right_child(i)] != None {
                next_index.push(self.right_child(i));
            }

            if self.root[self.left_child(i)] != None {
                next_index.push(self.left_child(i));
            }
        }

        return_vec
    }

    fn in_order(&self) -> Vec<T> {
        if self.root.is_empty() || self.root[0] == None {return vec![];}

        if self.next_index_out(0) && self.root[0] != None {
            return vec![self.root[0].unwrap()];
        }

        let mut return_vec: Vec<T> = Vec::new();
        let mut next_index: Vec<usize> = vec![0];
        let mut done_idx: Vec<bool> = vec![false; self.nodes as usize];

        let mut idx: usize;
        let num_nodes = self.num_nodes() as usize;

        while !next_index.is_empty() {
            idx = next_index.pop().unwrap();

            // Is next nodes outside the vec, add parent node to stack
            if self.next_index_out(idx) {
                done_idx[idx] = true;
                return_vec.push(self.root[idx].unwrap());
                idx = self.parent(idx);
            }

            // Add left child to stack so the value can be added later
            if self.root[self.left_child(idx)] != None && !done_idx[self.left_child(idx)] {
                next_index.push(self.left_child(idx));
                continue;
            }

            // If node not visited before, add the value to output vector
            if !done_idx[idx] {
                return_vec.push(self.root[idx].unwrap());
                done_idx[idx] = true;
            }

            // Add right child to stack so the value can be added later
            if self.root[self.right_child(idx)] != None && !done_idx[self.right_child(idx)] {
                next_index.push(self.right_child(idx));
                continue;
            }

            // Add the parent to the stack to move up the tree
            if idx != 0 {
                next_index.push(self.parent(idx));
            }

            // If all nodes are visited return the vector
            if num_nodes <= return_vec.len() {return return_vec;}
        }

        return_vec
    }

    fn post_order(&self) -> Vec<T> {
        if self.root.is_empty() || self.root[0] == None {return vec![];}

        if self.next_index_out(0) && self.root[0] != None {
            return vec![self.root[0].unwrap()];
        }

        let mut return_vec: Vec<T> = Vec::new();
        let mut next_index: Vec<usize> = vec![0];
        let mut done_idx: Vec<bool> = vec![false; self.nodes as usize];

        let mut idx: usize;
        let num_nodes = self.num_nodes() as usize;

        while !next_index.is_empty() {
            idx = next_index.pop().unwrap();

            // Is next nodes outside the vec, add parent node to stack
            if self.next_index_out(idx) {
                done_idx[idx] = true;
                return_vec.push(self.root[idx].unwrap());
                idx = self.parent(idx);
            }

            // Add left child to stack so the value can be added later
            if self.root[self.left_child(idx)] != None && !done_idx[self.left_child(idx)] {
                next_index.push(self.left_child(idx));
                continue;
            }

            // Add right child to stack so the value can be added later
            if self.root[self.right_child(idx)] != None && !done_idx[self.right_child(idx)] {
                next_index.push(self.right_child(idx));
                continue;
            }

            // If node not visited before, add the value to output vector
            if !done_idx[idx] {
                return_vec.push(self.root[idx].unwrap());
                done_idx[idx] = true;
            }

            // Add the parent to the stack to move up the tree
            if idx != 0 {
                next_index.push(self.parent(idx));
            }

            // If all nodes are visited return the vector
            if num_nodes <= return_vec.len() {
                return return_vec;
            }
        }

        return_vec
    }

    fn bfs(&self) -> Vec<T> {
        if self.root.is_empty() || self.root[0] == None {return vec![];}

        if self.next_index_out(0) {return vec![self.root[0].unwrap()];}

        let mut return_vec: Vec<T> = vec![self.root[0].unwrap()];
        let mut index_queue: VecDeque<usize> = VecDeque::new();

        self.add_children_to_queue(0, &mut index_queue);

        let mut current: usize;

        while !index_queue.is_empty() {
            current = index_queue.pop_front().unwrap();

            return_vec.push(self.root[current].unwrap());

            if self.index_out(current) {continue;}

            self.add_children_to_queue(current, &mut index_queue);
        }

        return_vec
    }
}

impl<T> TraversalFrom<T> for BinaryTree<T>
where T: PartialOrd, T: Copy, T: Display {
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

            if self.root[self.right_child(i)] != None {
                next_index.push(self.right_child(i));
            }

            if self.root[self.left_child(i)] != None {
                next_index.push(self.left_child(i));
            }
        }

        return_vec
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
                i = self.parent(i);
            }

            // Add left child to stack so the value can be added later
            if self.root[self.left_child(i)] != None && !done_idx[self.left_child(i)] {
                next_index.push(self.left_child(i));
                continue;
            }

            // If node not visited before, add the value to output vector
            if !done_idx[i] {
                return_vec.push(self.root[i].unwrap());
                done_idx[i] = true;
            }

            // Add right child to stack so the value can be added later
            if self.root[self.right_child(i)] != None && !done_idx[self.right_child(i)] {
                next_index.push(self.right_child(i));
                continue;
            }

            // Add the parent to the stack to move up the tree
            if i > idx {
                next_index.push(self.parent(i));
            }

            // If all nodes are visited return the vector
            if num_nodes <= return_vec.len() {return return_vec;}
        }

        return_vec
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
                i = self.parent(i);
            }

            // Add left child to stack so the value can be added later
            if self.root[self.left_child(i)] != None && !done_idx[self.left_child(i)] {
                next_index.push(self.left_child(i));
                continue;
            }

            // Add right child to stack so the value can be added later
            if self.root[self.right_child(i)] != None && !done_idx[self.right_child(i)] {
                next_index.push(self.right_child(i));
                continue;
            }

            // If node not visited before, add the value to output vector
            if !done_idx[i] {
                return_vec.push(self.root[i].unwrap());
                done_idx[i] = true;
            }

            // Add the parent to the stack to move up the tree
            if i > idx {
                next_index.push(self.parent(i));
            }

            // If all nodes are visited return the vector
            if num_nodes <= return_vec.len() {
                return return_vec;
            }
        }

        return_vec
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