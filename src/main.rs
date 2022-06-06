#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use crate::unbalanced::array::binary_tree::BinaryTree;
use crate::unbalanced::array::Insert;
use crate::utils::util::Node;

mod unbalanced;
mod utils;

fn main() {
    let mut root = BinaryTree {
        root: vec![Some(8)],
        nodes: 1,
        height: 1
    };

    root.insert_vec(0, vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

    for x in root.root {
        if x != None {
            print!("{} ", x.unwrap())
        } else {
            print!("_ ")
        }
    }
}
