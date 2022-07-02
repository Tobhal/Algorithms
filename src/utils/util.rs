use crate::unbalanced::array::binary_tree::BinaryTree;
use crate::balanced::array::avl_tree::AVLTree;

use core::fmt::Display;
use std::collections::VecDeque;

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