#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use crate::unbalanced::array::binary_tree::BinaryTree;
use crate::unbalanced::{Counting, Insert};
use crate::utils::util::Node;

mod unbalanced;
mod utils;

fn main() {
    let mut root = BinaryTree::new();

    root.insert_vec(vec![8, 3, 1, 6, 4, 7, 10, 14, 13, 13]);

    for x in &root.root {
        if *x != None {
            print!("{} ", x.unwrap())
        } else {
            print!("_ ")
        }
    }

    println!();
    println!("Nodes :       {}", root.num_nodes());
    println!("Leaves:       {}", root.num_leaves());
    println!("Two children: {}", root.num_two_children());
    println!("Levels:       {}", root.num_levels());
}
