use std::arch::aarch64::poly8x8_t;
use std::collections::VecDeque;
use std::env::current_exe;
use std::fmt::{Debug, Display};
use std::process::id;
use std::thread::current;
use crate::array::binary_tree::BinaryTree;
use crate::{impl_BFS, impl_counting, impl_ordered_traversal, impl_util, impl_utils};
use crate::utils::util::{
    Counting, Utility, OrderedTraversal, BFS, Util, Insert, InsertAt, Contains
};

// https://cmps-people.ok.ubc.ca/ylucet/DS/AVLtree.html

pub(crate) enum Direction {
    LEFT,
    RIGHT
}

impl Direction {
    fn get_direction_func(dir: Direction) -> (fn(usize) -> usize, fn(usize) -> usize) {
        match dir {
            Direction::LEFT => (AVLTree::<usize>::left_child, AVLTree::<usize>::right_child),
            Direction::RIGHT => (AVLTree::<usize>::right_child, AVLTree::<usize>::left_child)
        }
    }
}

impl From<bool> for Direction {
    fn from(b: bool) -> Self {
        match b {
            true => Direction::LEFT,
            false => Direction::RIGHT
        }
    }
}

pub(crate) trait Child {
    fn left_child(self) -> usize;
    fn right_child(self) -> usize;
    fn parent(self) -> usize;
}

impl Child for usize {
    // Replace AVLTree::<T>::left_child(idx)
    fn left_child(self) -> usize {
        2 * self + 1
    }

    // Replace AVLTree::<T>::right_child(idx)
    fn right_child(self) -> usize {
        2 * self + 2
    }

    // Replace AVLTree::<T>::parent(idx)
    fn parent(self) -> usize {
        (self - 1) / 2
    }
}

// https://www.cs.usfca.edu/~galles/visualization/AVLtree.html

#[derive(Debug)]
pub struct AVLTree<T> {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) balance_factor: Vec<u32>, // "height" of node*
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

impl_utils!(AVLTree<T: PartialOrd>);
impl_counting!(AVLTree<T: PartialOrd>);
impl_ordered_traversal!(AVLTree<T: PartialOrd + Copy>);
impl_BFS!(AVLTree<T: PartialOrd + Copy>);

impl<T> AVLTree<T> {
    pub(crate) fn new() -> AVLTree<T> {
        AVLTree {
            root: vec![],
            balance_factor: vec![],
            nodes: 0,
            height: 0
        }
    }

}

impl<T> AVLTree<T>
where T: Debug {
    pub(crate) fn println(&self) {
        self.root.iter().for_each(|e| match e {
            None => print!("_ "),
            Some(v) => print!("{:?} ", v)
        });
        println!();
    }
}

impl<T> AVLTree<T>
where T: PartialOrd + Copy + Debug {
    pub(crate) fn new_from_vec(vec: Vec<T>) -> AVLTree<T> {
        let mut root = AVLTree::new();
        root.insert_vec(vec);
        root
    }
}

impl<T> Insert<T> for AVLTree<T>
where T: PartialOrd + Copy + Debug {
    fn insert(&mut self, data: T) {
        if self.root.len() == 0 {
            self.root.push(Some(data));
            self.balance_factor.push(1);
            return;
        }

        let mut node_idx: usize = 0;

        // Insert node
        loop {
            if node_idx >= self.root.len() {
                self.increase_levels(1);
            }

            match self.root[node_idx] {
                None => {
                    println!("Insert {data:?} in {node_idx}");
                    self.root[node_idx] = Some(data);
                    self.balance_factor[node_idx] = 1;
                    break;
                }
                Some(val) => {
                    node_idx = if val > data {
                        AVLTree::<T>::left_child(node_idx)
                    } else {
                        AVLTree::<T>::right_child(node_idx)
                    };
                }
            }
        }

        let mut left_node: usize = 0;
        let mut right_node: usize = 0;

        let mut decrease_size = false;

        let mut last_balance_factor: u32 = 1;

        // Update balance factor and rotate
        while node_idx != 0 {
            node_idx = AVLTree::<T>::parent(node_idx);

            if last_balance_factor == self.balance_factor[node_idx] {
                self.balance_factor[node_idx] += 1;
                last_balance_factor = self.balance_factor[node_idx];
            }

            left_node = AVLTree::<T>::left_child(node_idx);
            right_node = AVLTree::<T>::right_child(node_idx);

            if self.balance_factor[left_node] > (self.balance_factor[right_node] + 1) {
                println!("Rotate right");
                self.rotate(node_idx, Direction::RIGHT);
                last_balance_factor -= 1;
                decrease_size = true;
            } else if self.balance_factor[right_node] > (self.balance_factor[left_node] + 1) {
                println!("Rotate left");
                self.rotate(node_idx, Direction::LEFT);
                last_balance_factor -= 1;
                decrease_size = true
            }
        }

        if decrease_size {
            self.decrease_levels(1);
        }
    }

    fn insert_vec(&mut self, data: Vec<T>) {
        for d in data {
            self.insert(d);
        }
    }
}

pub(crate) trait Rotate {
    fn rotate(&mut self, idx: usize, dir: Direction);

    fn move_nodes(&mut self, from_idx: usize, to_idx: usize);

    fn node_is_leaf(&self, node: usize, moved_to_indexes: &Vec<usize>) -> bool;
}

impl<T> Rotate for AVLTree<T>
where T: PartialOrd + Copy + Debug {
    fn rotate(&mut self, idx: usize, dir: Direction) {
        let (dir_towards, dir_against): (fn(usize) -> usize, fn(usize) -> usize) = Direction::get_direction_func(dir);

        if self.root[dir_towards(idx)] != None {
            self.move_nodes(dir_towards(idx), dir_towards(dir_towards(idx)))
        }

        self.root[dir_towards(idx)] = self.root[idx];
        self.root[idx] = None;

        self.balance_factor[dir_towards(idx)] = self.balance_factor[idx] - 2;
        self.balance_factor[idx] = 0;

        // println!("{:?}", !self.index_out(dir_against(dir_towards(idx))));
        // println!("{:?}", self.root[dir_against(dir_towards(idx))] == None);
        // println!("{:?}", !self.index_out(dir_against(dir_towards(idx))) && self.root[dir_against(dir_towards(idx))] == None);

        if !self.index_out(dir_against(dir_towards(idx))) && self.root[dir_against(dir_towards(idx))] == None {
            if dir_towards(dir_against(idx)) > dir_towards(idx) {
                self.move_nodes(
                    dir_towards(dir_against(idx)),
                    dir_against(dir_towards(idx))
                )
            } else {
                self.move_nodes(
                    dir_towards(dir_against(idx)),
                    dir_towards(dir_towards(idx))
                );
            }
        }

        let mut parent_idx = idx;
        let mut current_node = dir_against(idx);


        // Note: Implement BFS for moving the nodes. So:
        //  1. Append all children to a list
        //  2. Move the children
        //  3. Go to the childrens location and append their children
        //  4. If there are any children repeat until no more children or index_out()
        // Move nodes up
        loop {
            parent_idx = AVLTree::<T>::parent(current_node);

            println!("\t{:?} -> {:?}*", self.root[current_node], self.root[parent_idx]);

            // Move current node 1 step up
            self.root[parent_idx] = self.root[current_node];
            self.balance_factor[parent_idx] = self.balance_factor[current_node];

            self.root[current_node] = None;
            self.balance_factor[current_node] = 0;

            // Note: The problem is here
            // If current node is out of bounce, break
            if self.index_out(current_node) {
                break;
            }

            current_node = dir_against(current_node);
        }
    }

    fn move_nodes(&mut self, from: usize, to: usize) {
        println!("Move node({:?}) from {from} to {to}({:?})", self.root[from], self.root[to]);

        let mut from_idx: usize = from;
        let mut to_idx: usize = to;

        let mut moved_to_indexes: Vec<usize> = Vec::new();
        
        let mut child_towards: usize = 0;
        let mut child_against: usize = 0;

        let (dir_towards, dir_against): (fn(usize) -> usize, fn(usize) -> usize) = Direction::get_direction_func(Direction::from(to % 2 == 0));

        while self.root[from] != None {
            match self.root[from_idx] {
                None => {
                    from_idx = AVLTree::<T>::parent(from_idx);
                    to_idx = AVLTree::<T>::parent(to_idx);
                }
                Some(val) => {
                    child_towards = dir_towards(from_idx);
                    child_against = dir_against(from_idx);

                    // Move leafs
                    if self.node_is_leaf(from_idx, &moved_to_indexes) {
                        if to_idx >= self.root.len() {
                            self.increase_levels(1);
                        }

                        println!("\t{:?} -> {:?}", self.root[from_idx], self.root[to_idx]);

                        self.root[to_idx] = self.root[from_idx];
                        self.balance_factor[to_idx] = self.balance_factor[from_idx];

                        self.root[from_idx] = None;
                        self.balance_factor[from_idx] = 0;

                        moved_to_indexes.push(to_idx);

                    } else if self.root[child_towards] != None && !moved_to_indexes.contains(&child_towards) {
                        from_idx = dir_towards(from_idx);
                        to_idx = dir_towards(to_idx);
                    } else if self.root[child_against] != None && !moved_to_indexes.contains(&child_against) {
                        from_idx = dir_against(from_idx);
                        to_idx = dir_against(to_idx);
                    }
                }
            }
        }
    }

    fn node_is_leaf(&self, node: usize, moved_to_indexes: &Vec<usize>) -> bool {
        // TODO: remove variables?
        let left = AVLTree::<T>::left_child(node);
        let right = AVLTree::<T>::right_child(node);
        let len = self.root.len();

        if left > len || right > len {
            return true;
        }

        return (self.root[left] == None || moved_to_indexes.contains(&left)) &&
            (self.root[right] == None || moved_to_indexes.contains(&right));
    }
}

impl<T> Contains<T> for AVLTree<T>
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

impl<T> Util<T> for AVLTree<T>
    where T: PartialOrd + Copy {
    fn clear_from(self: &mut AVLTree<T>, idx: usize) {
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
        self.height += amount;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None);
        self.balance_factor.resize_with(self.nodes as usize, || 0);
    }

    fn decrease_levels(&mut self, amount: u32) {
        self.height -= amount;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None);
        self.balance_factor.resize_with(self.nodes as usize, || 0);
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

/*
Rotate temp code:
 */

/*

        /*
        let mut leftChild: usize = 0;
        let mut rightChild: usize = 0;

        let mut level: u32 = 0;

        // Move children of left sub-tree. TOOD: Replace with `loop` when done
        for i in 0..100 {
            leftChild = AVLTree::<T>::left_child(currentNode);
            rightChild = AVLTree::<T>::right_child(currentNode);

            // Case 1: Node have no children
            //  Move back up
            if self.next_index_out(currentNode) ||
                (self.root[leftChild].is_none() && self.root[rightChild].is_none()) {



                currentNode = AVLTree::<T>::parent(currentNode);
                level -= 1;
            }

            // Case 2: Node have 2 children
            // Case 2.1: left child


            // Case 2.2: right child



            // Case 3: Node have 1 child

        }

        // Move left sub tree for right sub tree of parent (right-left-subtree), replace with loop
        for i in 0..100 {

        }

         */
            rightChild = AVLTree::<T>::right_child(currentNode);
            leftChild = AVLTree::<T>::left_child(currentNode);

            if self.index_out(rightChild) || self.root[rightChild] != None {
                // Move down right
                currentNode = rightChild
            } else if self.index_out(leftChild) || self.root[leftChild] != None {
                // Move down left
                currentNode = leftChild
            }
 */

/*
impl<T> InsertAt<T> for AVLTree<T>
where T: PartialOrd + Copy + Display {
    fn insert_at(&mut self, idx: usize, data: T) {
        /*
        Do normal insert
         */
        if self.root.len() == 0 {
            self.root.push(Some(data));
            self.balance_factor[0] = 0;
            return;
        }

        let mut i = idx;

        loop {
            if i >= self.root.len() {
                self.increase_levels(1);
            }

            if self.root[i] == None && self.balance_factor[i] == 0 {
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
        self.balance_factor[i] = 0;
        let mut _prev_data = &data;
        /*
        loop {
            i = BinaryTree::<T>::parent(i);

            // update parent node balance factor
            if self.root[i].unwrap() > prev_data {
                self.balance_factor -= 1;
            } else {
                self.balance_factor += 1;
            }

            if self.balance_factor[i] > 2 {
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
        self.balance_factor.resize_with(self.nodes as usize, || 0);
    }

    fn get_child(&self, idx: usize, data: T) -> Result<usize, String> {
        todo!()
    }
}
 */