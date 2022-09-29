#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use crate::unbalanced::{Insert, Remove, Traversal};
use crate::unbalanced::array::{InsertAt, TraversalFrom};
use crate::unbalanced::array::binary_tree::BinaryTree;
use crate::unbalanced::array::graph::{Node, Graph};
use crate::utils::util::{OrderedTraversal, BFS};

mod balanced;
mod unbalanced;
mod utils;
mod tests;

fn main() {
    let mut node0 = Node::new(1);
    let mut node1 = Node::new(2);
    let mut node2 = Node::new(3);

    node0.add_children(vec![(0, 1), (1, 2)]);
    node1.add_children(vec![(0, 3), (2, 2)]);
    node2.add_children(vec![(1, 4), (0, 2)]);

    let mut graph = Graph::new();
    graph.add_nodes(vec![node0, node1, node2]);

    let graph2: Graph<char> = Graph::read_file("files/TestGraphData.txt");

    println!("{:#?}\n", graph);
    println!("{:#?}", graph2);
}
