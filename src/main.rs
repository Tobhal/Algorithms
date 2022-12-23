#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use crate::array::avl_tree::AVLTree;
use crate::utils::util::Utility;

mod utils;
mod tests;
mod array;
mod examples;

fn main() {
    let root: AVLTree<u8> = AVLTree {
        root: vec![Some(5), Some(1)],
        balanceFactor: vec![],
        nodes: 0,
        height: 0
    };
}
