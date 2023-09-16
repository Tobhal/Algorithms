use std::collections::VecDeque;

/*
#[macro_export]
macro_rules! _ {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? _ for $name $(< $( $lt ),+ >)? {

        }
    }
}

#[macro_export]
macro_rules! _ {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? _$(< $( $lt ),+ >)? for $name $(< $( $lt ),+ >)? {

        }
    }
}
 */

pub(crate) trait Counting {
    fn num_nodes(&self) -> u32;
    fn num_leaves(&self) -> u32;
    fn num_two_children(&self) -> u32;
    fn num_levels(&self) -> u32;
}

pub(crate) trait Insert<T> {
    fn insert(&mut self, data: T);
    fn insert_vec(&mut self, data: Vec<T>);
}

pub(crate) trait InsertAt<T> {
    fn insert_at(&mut self, idx: usize, data: T);
    fn insert_vec_at(&mut self, idx: usize, data: Vec<T>);
}

pub(crate) trait Util<T> {
    fn clear_from(&mut self, idx: usize);
    fn increase_levels(&mut self, amount: u32);
    fn decrease_levels(&mut self, amount: u32);
    fn get_child(&self, idx: usize, data: T) -> Result<usize, String>;
}

#[macro_export]
macro_rules! impl_util {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? Util for $name $(< $( $lt ),+ >)? {
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
    }
}

pub(crate) trait Contains<T> {
    fn contains(&self, data: T) -> bool;
    fn find(&self, data: T) -> Result<usize, String>;
}

#[macro_export]
macro_rules! impl_contains {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? Contains$(< $( $lt ),+ >)? for $name $(< $( $lt ),+ >)? {
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
    }
}

pub(crate) trait Remove<T> {
    fn remove(&mut self, data: T);
}

#[macro_export]
macro_rules! impl_counting {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? Counting for $name $(< $( $lt ),+ >)? {
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

                    if self.root[$name::<T>::left_child(current)] == None && self.root[$name::<T>::right_child(current)] == None {
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

                    if self.root[$name::<T>::left_child(current)] != None && self.root[BinaryTree::<T>::right_child(current)] != None {
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
    }
}

pub(crate) trait Utility {
    fn index_out(&self, idx: usize) -> bool;
    fn next_index_out(&self, idx: usize) -> bool;

    fn index_out_f(&self, idx: usize, op: fn (bool, bool) -> bool) -> bool;
    fn next_index_out_f(&self, idx: usize, op: fn (bool, bool) -> bool) -> bool;

    fn left_child(idx: usize) -> usize;
    fn right_child(idx: usize) -> usize;
    fn parent(idx: usize) -> usize;

    fn add_children_to_queue(&self, idx: usize, queue: &mut VecDeque<usize>);
}

#[macro_export]
macro_rules! impl_utils {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? Utility for $name $(< $( $lt ),+ >)? {
            fn index_out(&self, idx: usize) -> bool {
                $name::<T>::left_child(idx) > self.root.len() || $name::<T>::right_child(idx) > self.root.len() || idx > self.root.len()
            }

            fn next_index_out(&self, idx: usize) -> bool {
                $name::<T>::left_child(idx) > self.root.len() || $name::<T>::right_child(idx) > self.root.len()
            }

            fn index_out_f(&self, idx: usize, op: fn(bool, bool) -> bool) -> bool {
                op(
                    op(
                        $name::<T>::left_child(idx) > self.root.len(), $name::<T>::right_child(idx) > self.root.len()),
                    usize::from(idx) > self.root.len()
                )
            }

            fn next_index_out_f(&self, idx: usize,  op: fn(bool, bool) -> bool) -> bool {
                op($name::<T>::left_child(idx) > self.root.len(), $name::<T>::right_child(idx) > self.root.len())
            }

            fn left_child(idx: usize) -> usize {2 * idx  + 1}
            fn right_child(idx: usize) -> usize {2 * idx  + 2}
            fn parent(idx: usize) -> usize {(idx - 1) / 2}

            fn add_children_to_queue(&self, idx: usize, q: &mut VecDeque<usize>) {
                if self.root[$name::<T>::left_child(idx)] != None {
                    q.push_back($name::<T>::left_child(idx));
                }
                if self.root[$name::<T>::right_child(idx)] != None {
                    q.push_back($name::<T>::right_child(idx));
                }
            }
        }
    }
}

pub(crate) trait OrderedTraversal<T> {
    fn pre_order(&self) -> Vec<T>;
    fn pre_order_from(&self, idx: usize) -> Vec<T>;

    fn in_order(&self) -> Vec<T>;
    fn in_order_from(&self, idx: usize) -> Vec<T>;

    fn post_order(&self) -> Vec<T>;
    fn post_order_from(&self, idx: usize) -> Vec<T>;
}

#[macro_export]
macro_rules! impl_ordered_traversal {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? OrderedTraversal$(< $( $lt ),+ >)? for $name $(< $( $lt ),+ >)? {
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

                    if self.root[$name::<T>::right_child(i)] != None {
                        next_index.push($name::<T>::right_child(i));
                    }

                    if self.root[$name::<T>::left_child(i)] != None {
                        next_index.push($name::<T>::left_child(i));
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
                        i = $name::<T>::parent(i);
                    }

                    // Add left child to stack so the value can be added later
                    if self.root[$name::<T>::left_child(i)] != None && !done_idx[$name::<T>::left_child(i)] {
                        next_index.push($name::<T>::left_child(i));
                        continue;
                    }

                    // If node not visited before, add the value to output vector
                    if !done_idx[i] {
                        return_vec.push(self.root[i].unwrap());
                        done_idx[i] = true;
                    }

                    // Add right child to stack so the value can be added later
                    if self.root[$name::<T>::right_child(i)] != None && !done_idx[$name::<T>::right_child(i)] {
                        next_index.push($name::<T>::right_child(i));
                        continue;
                    }

                    // Add the parent to the stack to move up the tree
                    if i > idx {
                        next_index.push($name::<T>::parent(i));
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
                        i = $name::<T>::parent(i);
                    }

                    // Add left child to stack so the value can be added later
                    if self.root[$name::<T>::left_child(i)] != None && !done_idx[$name::<T>::left_child(i)] {
                        next_index.push($name::<T>::left_child(i));
                        continue;
                    }

                    // Add right child to stack so the value can be added later
                    if self.root[$name::<T>::right_child(i)] != None && !done_idx[$name::<T>::right_child(i)] {
                        next_index.push($name::<T>::right_child(i));
                        continue;
                    }

                    // If node not visited before, add the value to output vector
                    if !done_idx[i] {
                        return_vec.push(self.root[i].unwrap());
                        done_idx[i] = true;
                    }

                    // Add the parent to the stack to move up the tree
                    if i > idx {
                        next_index.push($name::<T>::parent(i));
                    }

                    // If all nodes are visited return the vector
                    if num_nodes <= return_vec.len() {
                        return return_vec;
                    }
                }

                return_vec
            }
        }
    }
}

pub(crate) trait BFS<T> {
    fn bfs(&self) -> Vec<T>;
    fn bfs_from(&self, idx: usize) -> Vec<T>;
}

#[macro_export]
macro_rules! impl_BFS {
    ( $name:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+ >)? ) => {
        impl $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? BFS$(< $( $lt ),+ >)? for $name $(< $( $lt ),+ >)? {
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
    }
}
