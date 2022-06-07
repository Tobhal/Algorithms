#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use crate::unbalanced::array::binary_tree::BinaryTree;
use crate::unbalanced::{Counting, Insert, Remove, Traversal};
use crate::unbalanced::array::TraversalFrom;
use crate::utils::util::Node;

mod unbalanced;
mod utils;
mod tests;

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

    println!("\n");
    println!("Order:");
    print!("Pre:  ");
    for e in root.pre_order_from(2) { print!("{} ", e); }

    println!();
    print!("In:   ");
    for e in root.in_order() { print!("{} ", e); }

    println!();
    print!("Post: ");
    for e in root.post_order() { print!("{} ", e); }

    println!();
    print!("BFS:  ");
    for e in root.bfs() { print!("{} ", e); }
}
