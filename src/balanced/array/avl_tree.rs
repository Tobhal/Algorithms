use std::collections::VecDeque;
use std::fmt::Display;
use crate::{BinaryTree, Insert};
use crate::unbalanced::array::Util;
use crate::utils::util::Utility;

pub struct AVLTree<T>
where T: PartialOrd, T: Copy, T: Display {
    pub(crate) root: Vec<Option<T>>,
    pub(crate) balanceFactor: Vec<i16>,
    pub(crate) nodes: u32,
    pub(crate) height: u32
}

crate::impl_utils!(AVLTree<T: PartialOrd + Copy + Display>);


impl<T> Insert<T> for AVLTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn insert(&mut self, data: T) {

    }

    fn insert_vec(&mut self, data: Vec<T>) {

    }
}

impl<T> Util<T> for AVLTree<T>
where T: PartialOrd, T: Copy, T: Display {
    fn clear_from(&mut self, idx: usize) {
        todo!()
    }

    fn increase_levels(&mut self, amount: u32) {
        self.height += 1;
        self.nodes = 2_i32.pow(self.height + amount) as u32 - 1;

        self.root.resize_with(self.nodes as usize, || None);
        self.balanceFactor.resize_with(self.nodes as usize, || 0);
    }
}